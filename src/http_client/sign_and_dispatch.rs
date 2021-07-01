use std::time::Duration;

use super::{errors::HttpResult, responses::HttpResponse, SignedRequest};

use async_trait::async_trait;

#[async_trait]
pub trait SignAndDispatch {
    async fn sign_and_dispatch(
        &self,
        mut request: SignedRequest,
        timeout: Option<Duration>,
    ) -> HttpResult<HttpResponse>;
}
