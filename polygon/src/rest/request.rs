use reqwest::{header::HeaderMap, Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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

pub(crate) trait RequestBuilderExt: Sized {
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
