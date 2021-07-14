use http_client::Params;
use hyper::{header::HeaderName, HeaderMap, Method};

use crate::OSS_META_PREFIX;

use super::{payload::Payload, Result, Schema};

pub type Metas = std::collections::BTreeMap<String, String>;

#[derive(Debug, Default)]
pub(crate) struct Request<'a> {
    /// Method or Verb
    method: Method,
    /// The bucket name containing the object
    bucket: Option<&'a str>,
    /// The name of the object to handle
    object: Option<&'a str>,
    /// The HTTP/HTTPS protocol
    schema: Schema,
    /// The Body Content
    payload: Option<Payload>,
    /// The query parameters
    params: Params,
    /// The request headers
    headers: HeaderMap,
}
impl<'a> Request<'a> {
    pub(crate) fn new(
        method: Method,
        bucket: Option<&'a str>,
        object: Option<&'a str>,
        schema: Schema,
        payload: Option<Payload>,
        params: Option<Params>,
        headers: Option<HeaderMap>,
    ) -> Self {
        Self {
            method: method.into(),
            bucket,
            object,
            schema,
            payload,
            params: params.unwrap_or_default(),
            headers: headers.unwrap_or_default(),
        }
    }
    pub(crate) fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    pub(crate) fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
    pub(crate) fn add_metas(&mut self, metas: Option<&Metas>) -> Result<()> {
        if let Some(metas) = metas {
            for (key, val) in metas {
                if key.starts_with(OSS_META_PREFIX) {
                    self.headers
                        .insert(key.parse::<HeaderName>()?, val.parse()?);
                } else {
                    self.headers.insert(
                        format!("{}{}", OSS_META_PREFIX, key).parse::<HeaderName>()?,
                        val.parse()?,
                    );
                };
            }
        }
        Ok(())
    }
    pub(crate) fn get_method(&self) -> &Method {
        &self.method
    }
    pub(crate) fn get_object(&self) -> Option<&str> {
        self.object
    }
    pub(crate) fn get_params(&self) -> &Params {
        &self.params
    }
    pub(crate) fn content_length(&self) -> usize {
        if let Some(payload) = &self.payload {
            payload.len().unwrap_or(0)
        } else {
            0
        }
    }
    pub(crate) fn take_payload(&mut self) -> Payload {
        self.payload.take().unwrap_or_default()
    }
    pub(crate) fn into_headers(self) -> HeaderMap {
        self.headers
    }
}
