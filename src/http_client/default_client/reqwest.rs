use std::time::Duration;

use crate::{
    http_client::{errors, signed_requests::SignedRequestPayload, SignedRequest},
    HttpError, HttpResponse, SignAndDispatch,
};

use async_trait::async_trait;

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
        let url = rqst.generate_url()?;
        let headers = rqst.headers().to_owned();
        let mut request_builder = self
            .request(method, url)
            .headers(headers)
            .query(rqst.params());
        if let Some(_duration) = timeout {
            request_builder = request_builder.timeout(_duration);
        }
        // if let Some(_payload) = rqst.payload {
        //     request_builder = request_builder.body();
        // }
        request_builder = match rqst.payload {
            Some(SignedRequestPayload::Buffer(_bytes)) => request_builder.body(_bytes),
            Some(SignedRequestPayload::Stream(_bytes)) => todo!(),
            None => request_builder,
        };
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
