use futures::TryStreamExt;
use hyper::{body::HttpBody, Body, HeaderMap, StatusCode};

pub struct HttpResponse {
    /// Status code of HTTP Request
    pub status: StatusCode,
    /// Contents of Response
    pub body: Body,
    /// Response headers
    pub headers: HeaderMap,
}

impl std::fmt::Debug for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpResponse")
            .field("StatusCode", &self.status.as_str())
            .field("Content: <ByteStream size_hint = >", &self.body.size_hint())
            .field("Headers", &self.headers)
            .finish()
    }
}
impl From<reqwest::Response> for HttpResponse {
    fn from(resp: reqwest::Response) -> Self {
        let status = resp.status();
        let headers = resp.headers().to_owned();
        let body = Body::wrap_stream(
            resp.bytes_stream()
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error)),
        );
        Self {
            status,
            headers,
            body,
        }
    }
}
impl From<hyper::Response<Body>> for HttpResponse {
    fn from(_: hyper::Response<Body>) -> Self {
        todo!()
    }
}
