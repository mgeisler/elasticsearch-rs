// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Delete Lifecycle API"]
pub enum SlmDeleteLifecycleParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b str),
}
impl<'b> SlmDeleteLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Slm Delete Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmDeleteLifecycleParts::PolicyId(ref policy_id) => {
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(encoded_policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Delete Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-delete-policy.html)\n\nDeletes an existing snapshot lifecycle policy."]
pub struct SlmDeleteLifecycle<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SlmDeleteLifecycleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SlmDeleteLifecycle<'a, 'b> {
    #[doc = "Creates a new instance of [SlmDeleteLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SlmDeleteLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SlmDeleteLifecycle {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Delete Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Execute Lifecycle API"]
pub enum SlmExecuteLifecycleParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b str),
}
impl<'b> SlmExecuteLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Slm Execute Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmExecuteLifecycleParts::PolicyId(ref policy_id) => {
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(encoded_policy_id.as_ref());
                p.push_str("/_execute");
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Execute Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-execute-lifecycle.html)\n\nImmediately creates a snapshot according to the lifecycle policy, without waiting for the scheduled time."]
pub struct SlmExecuteLifecycle<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SlmExecuteLifecycleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SlmExecuteLifecycle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmExecuteLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SlmExecuteLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SlmExecuteLifecycle {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmExecuteLifecycle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmExecuteLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Execute Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Execute Retention API"]
pub enum SlmExecuteRetentionParts {
    #[doc = "No parts"]
    None,
}
impl SlmExecuteRetentionParts {
    #[doc = "Builds a relative URL path to the Slm Execute Retention API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmExecuteRetentionParts::None => "/_slm/_execute_retention".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Execute Retention API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-execute-retention.html)\n\nDeletes any snapshots that are expired according to the policy's retention rules."]
pub struct SlmExecuteRetention<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SlmExecuteRetentionParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SlmExecuteRetention<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmExecuteRetention]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SlmExecuteRetention {
            client,
            parts: SlmExecuteRetentionParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmExecuteRetention<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmExecuteRetention {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Execute Retention API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Get Lifecycle API"]
pub enum SlmGetLifecycleParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b [&'b str]),
    #[doc = "No parts"]
    None,
}
impl<'b> SlmGetLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Slm Get Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmGetLifecycleParts::PolicyId(ref policy_id) => {
                let policy_id_str = policy_id.join(",");
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(encoded_policy_id.as_ref());
                p.into()
            }
            SlmGetLifecycleParts::None => "/_slm/policy".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Get Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-get-policy.html)\n\nRetrieves one or more snapshot lifecycle policy definitions and information about the latest snapshot attempts."]
pub struct SlmGetLifecycle<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SlmGetLifecycleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SlmGetLifecycle<'a, 'b> {
    #[doc = "Creates a new instance of [SlmGetLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SlmGetLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SlmGetLifecycle {
            client,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Get Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Get Stats API"]
pub enum SlmGetStatsParts {
    #[doc = "No parts"]
    None,
}
impl SlmGetStatsParts {
    #[doc = "Builds a relative URL path to the Slm Get Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmGetStatsParts::None => "/_slm/stats".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Get Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-get-stats.html)\n\nReturns global and policy-level statistics about actions taken by snapshot lifecycle management."]
pub struct SlmGetStats<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SlmGetStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SlmGetStats<'a, 'b> {
    #[doc = "Creates a new instance of [SlmGetStats]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SlmGetStats {
            client,
            parts: SlmGetStatsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Get Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Get Status API"]
pub enum SlmGetStatusParts {
    #[doc = "No parts"]
    None,
}
impl SlmGetStatusParts {
    #[doc = "Builds a relative URL path to the Slm Get Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmGetStatusParts::None => "/_slm/status".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Get Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-get-status.html)\n\nRetrieves the status of snapshot lifecycle management (SLM)."]
pub struct SlmGetStatus<'a, 'b> {
    client: &'a Elasticsearch,
    parts: SlmGetStatusParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SlmGetStatus<'a, 'b> {
    #[doc = "Creates a new instance of [SlmGetStatus]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SlmGetStatus {
            client,
            parts: SlmGetStatusParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Get Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Put Lifecycle API"]
pub enum SlmPutLifecycleParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b str),
}
impl<'b> SlmPutLifecycleParts<'b> {
    #[doc = "Builds a relative URL path to the Slm Put Lifecycle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmPutLifecycleParts::PolicyId(ref policy_id) => {
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_policy_id.len());
                p.push_str("/_slm/policy/");
                p.push_str(encoded_policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Put Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-put-policy.html)\n\nCreates or updates a snapshot lifecycle policy."]
pub struct SlmPutLifecycle<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SlmPutLifecycleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SlmPutLifecycle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmPutLifecycle] with the specified API parts"]
    pub fn new(client: &'a Elasticsearch, parts: SlmPutLifecycleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SlmPutLifecycle {
            client,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmPutLifecycle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmPutLifecycle {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Put Lifecycle API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Start API"]
pub enum SlmStartParts {
    #[doc = "No parts"]
    None,
}
impl SlmStartParts {
    #[doc = "Builds a relative URL path to the Slm Start API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmStartParts::None => "/_slm/start".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-start.html)\n\nTurns on snapshot lifecycle management (SLM)."]
pub struct SlmStart<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SlmStartParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SlmStart<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmStart]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SlmStart {
            client,
            parts: SlmStartParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmStart<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmStart {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Start API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Slm Stop API"]
pub enum SlmStopParts {
    #[doc = "No parts"]
    None,
}
impl SlmStopParts {
    #[doc = "Builds a relative URL path to the Slm Stop API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SlmStopParts::None => "/_slm/stop".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Builder for the [Slm Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-stop.html)\n\nTurns off snapshot lifecycle management (SLM)."]
pub struct SlmStop<'a, 'b, B> {
    client: &'a Elasticsearch,
    parts: SlmStopParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SlmStop<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SlmStop]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        let headers = HeaderMap::new();
        SlmStop {
            client,
            parts: SlmStopParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SlmStop<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SlmStop {
            client: self.client,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            source: self.source,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Slm Stop API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "source")]
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .client
            .send(method, &path, headers, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Snapshot Lifecycle Management APIs"]
pub struct Slm<'a> {
    client: &'a Elasticsearch,
}
impl<'a> Slm<'a> {
    #[doc = "Creates a new instance of [Slm]"]
    pub fn new(client: &'a Elasticsearch) -> Self {
        Self { client }
    }
    #[doc = "[Slm Delete Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-delete-policy.html)\n\nDeletes an existing snapshot lifecycle policy."]
    pub fn delete_lifecycle<'b>(
        &'a self,
        parts: SlmDeleteLifecycleParts<'b>,
    ) -> SlmDeleteLifecycle<'a, 'b> {
        SlmDeleteLifecycle::new(&self.client, parts)
    }
    #[doc = "[Slm Execute Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-execute-lifecycle.html)\n\nImmediately creates a snapshot according to the lifecycle policy, without waiting for the scheduled time."]
    pub fn execute_lifecycle<'b>(
        &'a self,
        parts: SlmExecuteLifecycleParts<'b>,
    ) -> SlmExecuteLifecycle<'a, 'b, ()> {
        SlmExecuteLifecycle::new(&self.client, parts)
    }
    #[doc = "[Slm Execute Retention API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-execute-retention.html)\n\nDeletes any snapshots that are expired according to the policy's retention rules."]
    pub fn execute_retention<'b>(&'a self) -> SlmExecuteRetention<'a, 'b, ()> {
        SlmExecuteRetention::new(&self.client)
    }
    #[doc = "[Slm Get Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-get-policy.html)\n\nRetrieves one or more snapshot lifecycle policy definitions and information about the latest snapshot attempts."]
    pub fn get_lifecycle<'b>(&'a self, parts: SlmGetLifecycleParts<'b>) -> SlmGetLifecycle<'a, 'b> {
        SlmGetLifecycle::new(&self.client, parts)
    }
    #[doc = "[Slm Get Stats API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-get-stats.html)\n\nReturns global and policy-level statistics about actions taken by snapshot lifecycle management."]
    pub fn get_stats<'b>(&'a self) -> SlmGetStats<'a, 'b> {
        SlmGetStats::new(&self.client)
    }
    #[doc = "[Slm Get Status API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-get-status.html)\n\nRetrieves the status of snapshot lifecycle management (SLM)."]
    pub fn get_status<'b>(&'a self) -> SlmGetStatus<'a, 'b> {
        SlmGetStatus::new(&self.client)
    }
    #[doc = "[Slm Put Lifecycle API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-put-policy.html)\n\nCreates or updates a snapshot lifecycle policy."]
    pub fn put_lifecycle<'b>(
        &'a self,
        parts: SlmPutLifecycleParts<'b>,
    ) -> SlmPutLifecycle<'a, 'b, ()> {
        SlmPutLifecycle::new(&self.client, parts)
    }
    #[doc = "[Slm Start API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-start.html)\n\nTurns on snapshot lifecycle management (SLM)."]
    pub fn start<'b>(&'a self) -> SlmStart<'a, 'b, ()> {
        SlmStart::new(&self.client)
    }
    #[doc = "[Slm Stop API](https://www.elastic.co/guide/en/elasticsearch/reference/7.7/slm-api-stop.html)\n\nTurns off snapshot lifecycle management (SLM)."]
    pub fn stop<'b>(&'a self) -> SlmStop<'a, 'b, ()> {
        SlmStop::new(&self.client)
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for Snapshot Lifecycle Management APIs"]
    pub fn slm(&self) -> Slm {
        Slm::new(&self)
    }
}
