use async_trait::async_trait;

use crate::{http_errors::HttpResult, HttpRequest, HttpResponse};

#[async_trait]
pub trait HttpClient {
    async fn dispatch(&self, request: HttpRequest) -> HttpResult<HttpResponse>;
}
