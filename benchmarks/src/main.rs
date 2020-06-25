use chrono::{DateTime, Duration, Utc};
use elasticsearch::{http::response::Response, Elasticsearch};
use std::{borrow::BorrowMut, env, error, fmt};
use tokio::runtime::Runtime;
#[macro_use]
extern crate serde_json;
extern crate rustc_version_runtime;
extern crate sys_info;
use crate::{
    actions::{index::action as index_action, ping::action as ping_action},
    record::{Git, Os, Target},
};
use elasticsearch::http::transport::Transport;
use once_cell::sync::Lazy;
use rustc_version_runtime::version;
use std::{collections::BTreeMap, time::Instant};

mod actions;
mod record;

static CLIENT_BENCHMARK_CATEGORY: Lazy<String> =
    Lazy::new(|| std::env::var("CLIENT_BENCHMARK_CATEGORY").unwrap_or_else(|_| "".to_string()));

static FILTER: Lazy<String> =
    Lazy::new(|| std::env::var("FILTER").unwrap_or_else(|_| "".to_string()));

fn main() -> Result<(), Error> {
    let rustc_version = version();
    let mut config = Config::new(rustc_version.to_string())?;

    let benchmarks = Benchmarks::new();
    let mut runtime = Runtime::new().unwrap();

    for operation in benchmarks.operations {
        if FILTER.contains(&operation.action) {
            continue;
        }

        let mut runner = Runner::new(&config, &operation);

        match runner.run() {
            Ok(_) => {}
            Err(e) => println!("{}", e.to_string()),
        }

        for stat in &runner.stats {
            println!(
                "{}: {}ns",
                &operation.action,
                stat.duration.num_nanoseconds().unwrap()
            )
        }
    }

    Ok(())
}

struct Benchmarks {
    operations: Vec<Action>,
}

impl Benchmarks {
    pub fn new() -> Self {
        Self {
            operations: vec![ping_action(), index_action()],
        }
    }
}

pub struct Config {
    build_id: String,
    environment: String,
    target: record::Target,
    runner: record::Runner,
    runner_client: Elasticsearch,
    report_client: Elasticsearch,
}

impl Config {
    pub fn new(rustc_version: String) -> Result<Self, Error> {
        let env_keys = vec![
            "BUILD_ID",
            "DATA_SOURCE",
            "CLIENT_BRANCH",
            "CLIENT_COMMIT",
            "CLIENT_BENCHMARK_ENVIRONMENT",
            "ELASTICSEARCH_TARGET_URL",
            "ELASTICSEARCH_REPORT_URL",
            "TARGET_SERVICE_TYPE",
            "TARGET_SERVICE_NAME",
            "TARGET_SERVICE_VERSION",
            "TARGET_SERVICE_OS_FAMILY",
        ];

        let (vars, errors): (Vec<_>, Vec<_>) = env_keys
            .iter()
            .map(|e| match std::env::var(e.clone()) {
                Ok(v) if !v.is_empty() => Ok((e.to_string(), v.clone())),
                Ok(v) => Err(Error::config(format!(
                    "{} environment variable is empty",
                    e
                ))),
                Err(err) => Err(Error::config(format!("{} {}", e, err.to_string()))),
            })
            .partition(Result::is_ok);

        if !errors.is_empty() {
            let errors: Vec<_> = errors
                .into_iter()
                .map(|e| e.unwrap_err().to_string())
                .collect();
            return Err(Error::config(errors.join("\n")));
        }

        let vars = vars
            .into_iter()
            .map(Result::unwrap)
            .collect::<BTreeMap<String, String>>();

        let service = record::Service {
            ty: vars.get("TARGET_SERVICE_TYPE").unwrap().to_string(),
            name: vars.get("TARGET_SERVICE_NAME").unwrap().to_string(),
            version: vars.get("TARGET_SERVICE_VERSION").unwrap().to_string(),
            git: Git {
                commit: vars.get("CLIENT_COMMIT").unwrap().to_string(),
                branch: vars.get("CLIENT_BRANCH").unwrap().to_string(),
            },
        };

        let os = Os {
            family: vars.get("TARGET_SERVICE_OS_FAMILY").unwrap().to_string(),
        };

        Ok(Self {
            build_id: vars.get("BUILD_ID").unwrap().to_string(),
            environment: vars
                .get("CLIENT_BENCHMARK_ENVIRONMENT")
                .unwrap()
                .to_string(),
            target: Target {
                service: service.clone(),
                os: os.clone(),
            },
            runner: record::Runner {
                service,
                runtime: record::Runtime {
                    name: "rust".to_string(),
                    version: rustc_version,
                },
                os,
            },
            runner_client: Elasticsearch::new(
                Transport::single_node(vars.get("ELASTICSEARCH_TARGET_URL").unwrap()).unwrap(),
            ),
            report_client: Elasticsearch::new(
                Transport::single_node(vars.get("ELASTICSEARCH_REPORT_URL").unwrap()).unwrap(),
            ),
        })
    }

    pub fn runner_client(&self) -> &Elasticsearch {
        &self.runner_client
    }

    pub fn environment(&self) -> &str {
        self.environment.as_str()
    }
}

struct ConfigOs {
    family: String,
}

struct ConfigGit {
    branch: String,
    commit: String,
}

struct ConfigService {
    ty: String,
    name: String,
    version: String,
    git: ConfigGit,
}

struct Stats {
    start: DateTime<Utc>,
    duration: Duration,
    outcome: String,
    status_code: Option<u16>,
}

struct Runner<'a> {
    config: &'a Config,
    stats: Vec<Stats>,
    action: &'a Action,
}

impl<'a> Runner<'a> {
    pub fn new(config: &'a Config, action: &'a Action) -> Self {
        Self {
            config,
            stats: Vec::new(),
            action,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let operations = self.action.operations.unwrap_or_else(|| 1);
        let category = self
            .action
            .category()
            .unwrap_or_else(|| CLIENT_BENCHMARK_CATEGORY.as_ref())
            .to_string();
        let environment = self
            .action
            .environment()
            .unwrap_or_else(|| self.config.environment())
            .to_string();

        let mut errors: Vec<String> = Vec::with_capacity(
            (self.action.warmups + (self.action.repetitions * operations)) as usize,
        );

        let client = self.config.runner_client();
        let mut runtime = Runtime::new().unwrap();

        match self.action.setup {
            Some(f) => {
                (f)(client, &mut runtime)?;
            }
            None => (),
        }

        for i in 0..self.action.warmups {
            match self.action.measure(i, client, &mut runtime) {
                Ok(r) => {
                    if !r.status_code().is_success() {
                        let e = r.error_for_status_code().err().unwrap();
                        errors.push(format!("warmup {}: {}", i, e.to_string()))
                    }
                }
                Err(e) => errors.push(format!("warmup {}: {}", i, e.to_string())),
            }
        }

        for i in 0..self.action.repetitions {
            let start = Utc::now();
            let now = Instant::now();
            let result = self.action.measure(i, client, &mut runtime);
            let duration = now.elapsed();
            let mut outcome = String::new();
            let mut status_code: Option<u16> = None;
            match result {
                Ok(r) => {
                    status_code = Some(r.status_code().as_u16());
                    if !r.status_code().is_success() {
                        let e = r.error_for_status_code().err().unwrap();
                        errors.push(format!("run {}: {}", i, e.to_string()));
                        outcome.push_str("failure");
                    } else {
                        outcome.push_str("success");
                    }
                }
                Err(e) => {
                    errors.push(format!("run {}: {}", i, e.to_string()));
                    outcome.push_str("failure");
                }
            }

            self.stats.push(Stats {
                start,
                duration: chrono::Duration::from_std(duration).unwrap(),
                outcome,
                status_code,
            });
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::run(errors))
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

impl Error {
    pub(crate) fn config(err: impl Into<String>) -> Self {
        Error {
            kind: Kind::Config(err.into()),
        }
    }

    pub(crate) fn run(errs: Vec<String>) -> Self {
        Error {
            kind: Kind::Run(errs),
        }
    }
}

impl From<elasticsearch::Error> for Error {
    fn from(err: elasticsearch::Error) -> Self {
        Error {
            kind: Kind::Response(err),
        }
    }
}

#[derive(Debug)]
enum Kind {
    Config(String),
    Response(elasticsearch::Error),
    Run(Vec<String>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            Kind::Config(_) => None,
            Kind::Response(err) => Some(err),
            Kind::Run(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::Config(err) => err.fmt(f),
            Kind::Response(err) => err.fmt(f),
            Kind::Run(errs) => errs.join("\n").fmt(f),
        }
    }
}

pub struct Action {
    action: String,
    category: Option<String>,
    environment: Option<String>,
    warmups: i32,
    repetitions: i32,
    operations: Option<i32>,
    setup: Option<fn(&Elasticsearch, &mut Runtime) -> Result<Response, elasticsearch::Error>>,
    run: fn(i32, &Elasticsearch, &mut Runtime) -> Result<Response, elasticsearch::Error>,
}

impl Action {
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }

    pub fn measure(
        &self,
        i: i32,
        client: &Elasticsearch,
        runtime: &mut Runtime,
    ) -> Result<Response, elasticsearch::Error> {
        (self.run)(i, client, runtime)
    }
}
