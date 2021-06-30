use std::time::Duration;

use super::{
    errors::{self, HttpError, HttpResult},
    responses::HttpResponse,
    SignedRequest,
};

use async_trait::async_trait;

#[async_trait]
pub trait SignAndDispatch {
    async fn sign_and_dispatch(
        &self,
        mut request: SignedRequest,
        timeout: Option<Duration>,
    ) -> HttpResult<HttpResponse>;
}

#[async_trait]
impl SignAndDispatch for reqwest::Client {
    async fn sign_and_dispatch(
        &self,
        rqst: SignedRequest,
        timeout: Option<Duration>,
    ) -> Result<HttpResponse, HttpError> {
        let mut rqst = rqst;
        rqst.oss_sign()?;
        let method = rqst.method().to_owned();
        let url = rqst.get_url().unwrap().to_owned();
        let headers = rqst.headers().to_owned();
        let mut request_builder = self
            .request(method, url)
            .headers(headers)
            .query(rqst.params());
        if let Some(_duration) = timeout {
            request_builder = request_builder.timeout(_duration);
        }
        if let Some(_payload) = rqst.payload {
            request_builder = request_builder.body(_payload.into_vec());
        }
        let ret = request_builder.send().await?;
        let http_resp = HttpResponse::from_resp(ret).await;
        Ok(http_resp)
    }
}
impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        errors::client(e)
    }
}

// impl From<reqwest::Response> for HttpResponse {
//     fn from(resp: reqwest::Response) -> Self {
//         let status = resp.status();
//         let headers = resp.headers().to_owned();
//         // let bytes = rt.block_on(resp.bytes()).unwrap_or_default();
//         // let bytes = Byt;
//         resp.bytes()

//         Self {
//             status,
//             headers,
//             body: Box::pin(bytes),
//         }
//     }
// }
