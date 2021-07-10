use async_trait::async_trait;
use hyper::{Body, Request};
use std::convert::TryFrom;

use crate::{http_errors::HttpResult, HttpClient, HttpRequest, HttpResponse};

#[async_trait]
impl HttpClient for reqwest::Client {
    async fn dispatch(&self, request: HttpRequest) -> HttpResult<HttpResponse> {
        let request: Request<Body> = request.into();
        let request = reqwest::Request::try_from(request)?;
        Ok(self.execute(request).await?.into())
    }
}
