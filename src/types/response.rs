use futures::StreamExt;
use http_client::HttpResponse;
use hyper::{HeaderMap, StatusCode};
use std::io::Error as IoError;

use crate::ByteStream;

pub struct Response {
    /// Status code of HTTP Request
    pub status: StatusCode,
    /// Contents of Response
    pub body: ByteStream,
    /// Response headers
    pub headers: HeaderMap,
}

impl From<HttpResponse> for Response {
    fn from(http_response: HttpResponse) -> Self {
        let HttpResponse {
            status,
            headers,
            body,
        } = http_response;
        let body = ByteStream::new(body.map(|try_chunk| {
            try_chunk.map(|c| c).map_err(|e| {
                IoError::new(
                    std::io::ErrorKind::Other,
                    format!("Error obtaining chunk: {}", e),
                )
            })
        }));
        Self {
            status,
            headers,
            body,
        }
    }
}
