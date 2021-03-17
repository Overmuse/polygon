use crate::errors::{Error, Result};
use crate::rest::{Request, RequestBuilderExt};
use futures::TryFutureExt;
use reqwest::Client as ReqwestClient;
use std::env;
use std::sync::Arc;

/// The main client used for making request to Alpaca.
///
/// `AlpacaConfig` stores an async Reqwest client as well as the associate
/// base url for the Alpaca server.
#[derive(Clone)]
pub struct Client {
    /// The underlying Reqwest client used for requests.
    inner: Arc<ReqwestClient>,
    /// The url to which the request are sent.
    url: String,
    /// The api token.
    token: String,
}

impl Client {
    /// Create a new `Client`.
    pub fn new(url: String, token: String) -> Self {
        let inner = Arc::new(ReqwestClient::new());

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

    /// Send a `Request` to Alpaca
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
