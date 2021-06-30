use super::{errors::HttpResult, *};
use http::{header::HeaderName, HeaderMap, HeaderValue, Method};
use url::Url;

#[derive(Clone, Debug, Default)]
pub struct SignedRequest {
    pub method: Method,
    pub region: Region,
    pub bucket: String,
    pub object: String,
    pub headers: HeaderMap,
    pub params: Params,
    pub payload: Option<Box<[u8]>>,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub url: Option<Url>,
    schema: Schema,
}
impl SignedRequest {
    pub fn new<M, S1, S2, S3, S4>(
        method: M,
        region: &Region,
        bucket: S1,
        object: S2,
        access_key_id: S3,
        access_key_secret: S4,
        schema: Schema,
    ) -> Self
    where
        M: Into<Method>,
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Self {
            method: method.into(),
            region: region.clone(),
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            bucket: bucket.into(),
            object: object.into(),
            schema,
            ..Default::default()
        }
    }
    /// Headers are kept sorted by key name with in BTreeMap
    pub fn add_headers<K, V>(&mut self, headers: impl IntoIterator<Item = (K, V)>)
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        for (k, v) in headers.into_iter() {
            self.add_header(k, v);
        }
    }
    pub(crate) fn add_header<K, V>(&mut self, key: K, value: V)
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.headers.insert(key.into(), value.into());
    }
    pub fn remove_header(&mut self, key: &str) {
        let key_lower = key.to_ascii_lowercase();
        self.headers.remove(&key_lower);
    }

    pub fn set_params(&mut self, params: Params) {
        self.params = params;
    }
    pub fn add_params<'b, K, V>(&mut self, key: K, val: V)
    where
        K: Into<String>,
        V: Into<Option<&'b str>>,
    {
        self.params
            .insert(key.into(), val.into().map(|s| s.to_owned()));
    }
    pub fn load<P>(&mut self, payload: P) -> usize
    where
        P: Into<Box<[u8]>>,
    {
        self.payload = Some(payload.into());
        self.payload.as_ref().unwrap().len()
    }
    pub fn unload(&mut self) -> Option<Box<[u8]>> {
        self.payload.take()
    }
    pub fn set_content_type<V>(&mut self, content_type: V)
    where
        V: Into<HeaderValue>,
    {
        self.add_header(http::header::CONTENT_TYPE, content_type.into())
    }
    pub fn set_content_length<V>(&mut self, content_length: V)
    where
        V: Into<HeaderValue>,
    {
        self.add_header(http::header::CONTENT_LENGTH, content_length.into())
    }
    pub fn set_schema<S: AsRef<str>>(&mut self, schema: S) {
        self.schema = schema.as_ref().parse().unwrap_or_default()
    }
    pub fn get_schema(&self) -> String {
        format!("{}", self.schema)
    }
    // /// Computes and sets the Content-MD5 header based on the current payload.
    // ///
    // /// Has no effect if the payload is not set, or is not a buffer. Will not
    // /// override an existing value for the `Content-MD5` header.
    // pub fn maybe_set_content_md5_header(&mut self) {
    //     if self.headers.contains_key("Content-MD5") {
    //         return;
    //     }
    //     if let Some(SignedRequestPayload::Buffer(ref payload)) = self.payload {
    //         let digest = Md5::digest(payload);
    //         self.add_header("Content-MD5", &base64::encode(&*digest));
    //     }
    // }

    pub fn get_url(&mut self) -> Option<Url> {
        self.generate_url().ok()
    }

    pub fn generate_url(&self) -> HttpResult<Url> {
        let url_str = if self.bucket.is_empty() {
            format!(
                "{}://{}/{}{}",
                self.get_schema(),
                self.region.endpoint(),
                self.object,
                get_params_str(&self.params),
            )
        } else {
            format!(
                "{}://{}.{}/{}{}",
                self.get_schema(),
                self.bucket,
                self.region.endpoint(),
                self.object,
                get_params_str(&self.params),
            )
        };
        Url::parse(&url_str).map_err(errors::url)
    }

    /// Set the signed request's method.
    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    /// Get a reference to the signed request's method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get a reference to the signed request's headers.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get a reference to the signed request's params.
    pub fn params(&self) -> &Params {
        &self.params
    }
}

fn get_params_str(params: &Params) -> String {
    let mut result = String::new();
    for (k, v) in params {
        if result.is_empty() {
            result += "?";
        } else {
            result += "&";
        }
        if let Some(_v) = v {
            result += &format!("{}={}", k, _v);
        } else {
            result += k;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_request_test() {
        let sr = SignedRequest::new(
            Method::GET,
            &Region::BeiJing,
            "dev-sheet-calced",
            "A",
            "",
            "",
            Schema::Http,
        );
        println!("{:?}", sr.generate_url());
    }
}
