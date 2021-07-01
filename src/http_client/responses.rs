use std::io;

use futures::StreamExt;
use http::{HeaderMap, StatusCode};

use super::stream::ByteStream;

/// Stores the response from a HTTP request.
pub struct HttpResponse {
    /// Status code of HTTP Request
    pub status: StatusCode,
    /// Contents of Response
    pub body: ByteStream,
    /// Response headers
    pub headers: HeaderMap,
}

impl std::fmt::Debug for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpResponse")
            .field("StatusCode", &self.status.as_str())
            .field(
                "Content: <ByteStream size_hint={:?}>",
                &self.body.size_hint(),
            )
            .field("Headers", &self.headers)
            .finish()
    }
}
impl HttpResponse {
    pub(crate) async fn from_resp(resp: reqwest::Response) -> Self {
        let status = resp.status();
        let headers = resp.headers().to_owned();
        // let bytes = resp.bytes().await.unwrap();
        Self {
            status,
            headers,
            body: ByteStream::new(
                resp.bytes_stream()
                    .map(|stream| stream.map_err(|e| io::Error::new(io::ErrorKind::Other, e))),
            ),
        }
    }
}
