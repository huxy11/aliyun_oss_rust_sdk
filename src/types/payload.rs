use bytes::Bytes;
use hyper::Body;

use crate::ByteStream;

/// Possible payloads included in a `Request`.
#[derive(Debug)]
pub enum Payload {
    /// Transfer payload in a single chunk
    Buffer(Bytes),
    /// Transfer payload in multiple chunks
    Stream(ByteStream),
}
impl Payload {
    /// Convert `SignedRequestPayload` into a hyper `Body`
    pub fn into_body(self) -> Body {
        match self {
            Payload::Buffer(bytes) => Body::from(bytes),
            Payload::Stream(stream) => Body::wrap_stream(stream),
        }
    }
    pub(crate) fn len(&self) -> Option<usize> {
        match self {
            Payload::Buffer(bytes) => Some(bytes.len()),
            Payload::Stream(stream) => stream.size_hint().1,
        }
    }
}

impl Default for Payload {
    fn default() -> Self {
        Self::Buffer(Bytes::new())
    }
}
