use async_trait::async_trait;
use hyper::{Body, Request};
use once_cell::sync::OnceCell;
use reqwest::Client;
use std::convert::TryFrom;

use crate::{http_errors::HttpResult, HttpClient, HttpRequest, HttpResponse};

// Reusable Lazy Initialized Global reqwest::Client
static REQWEST_CLIENT: OnceCell<reqwest::Client> = OnceCell::new();
pub fn default_client() -> Client {
    REQWEST_CLIENT.get_or_init(reqwest::Client::new).clone()
}

#[async_trait]
impl HttpClient for reqwest::Client {
    async fn dispatch(&self, request: HttpRequest) -> HttpResult<HttpResponse> {
        let request: Request<Body> = request.into();
        let request = reqwest::Request::try_from(request)?;
        Ok(self.execute(request).await?.into())
    }
}
