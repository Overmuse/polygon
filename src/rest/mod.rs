use crate::errors::{Error, Result};
use futures::future::TryFutureExt;
use reqwest::{header::HeaderMap, Client as ReqwestClient, Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;

pub mod reference;

pub enum RequestBody<T> {
    None,
    Query(T),
    Json(T),
}

impl<T> Default for RequestBody<T> {
    fn default() -> Self {
        RequestBody::None
    }
}

pub trait Request {
    type Body: Serialize;
    type Response: for<'de> Deserialize<'de>;
    const METHOD: Method = Method::GET;

    fn endpoint(&self) -> Cow<str>;

    fn headers(&self) -> HeaderMap {
        Default::default()
    }

    fn body(&self) -> RequestBody<&Self::Body> {
        Default::default()
    }
}

trait RequestBuilderExt: Sized {
    fn polygon_body<T: Serialize>(self, body: RequestBody<T>) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
    fn polygon_body<T: Serialize>(self, body: RequestBody<T>) -> Self {
        match body {
            RequestBody::None => self,
            RequestBody::Json(value) => self.json(&value),
            RequestBody::Query(value) => self.query(&value),
        }
    }
}

/// The main client used for making request to Alpaca.
///
/// `AlpacaConfig` stores an async Reqwest client as well as the associate
/// base url for the Alpaca server.
pub struct Client {
    /// The underlying Reqwest client used for requests.
    inner: ReqwestClient,
    /// The url to which the request are sent.
    url: String,
    /// The api token.
    token: String,
}

impl Client {
    /// Create a new `Client`.
    pub fn new(url: String, token: String) -> Self {
        let inner = ReqwestClient::new();

        Self { inner, url, token }
    }

    /// Creates a `Client` from environment variables.
    ///
    /// The three environment variables used to instantiate the struct are:
    /// - `POLYGON_BASE_URL`
    /// - `POLYGON_TOKEN`
    pub fn from_env() -> Result<Self> {
        let url = env::var("POLYGON_BASE_URL")?;
        let token = env::var("POLYGON_TOKEN")?;
        Ok(Self::new(url, token))
    }

    pub async fn send<R: Request>(&self, request: R) -> Result<R::Response> {
        let endpoint = request.endpoint();
        let endpoint = endpoint.trim_matches('/');
        let url = format!("{}/{}", self.url, endpoint);

        let res = self
            .inner
            .request(R::METHOD, &url)
            .headers(request.headers())
            .polygon_body(request.body())
            .query(&[("apiKey", &self.token)])
            .send()
            .await?;
        let status = res.status();
        if status.is_success() {
            res.json().map_err(From::from).await
        } else if status.is_client_error() {
            Err(Error::ClientError(status, res.text().await?))
        } else {
            Err(Error::ServerError(status, res.text().await?))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn test_from_env() {
        env::set_var("POLYGON_BASE_URl", "URL");
        env::set_var("POLYGON_TOKEN", "TOKEN");

        Client::from_env().unwrap();
    }
}
