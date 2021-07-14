use base64::encode;
use crypto::{hmac::Hmac, mac::Mac, sha1::Sha1};
use http_client::{HttpClient, HttpRequest, Params};
use hyper::header::HeaderValue;
use url::Url;

use std::{collections::BTreeMap, str::FromStr};

use crate::{
    auth::canonicalized_resource,
    statics::{CONTENT_MD5, CONTENT_TYPE, OSS_CANONICALIZED_PREFIX},
    types::{Region, Request, Result, Schema},
    Response,
};

#[derive(Debug)]
pub struct OSSClient<C: HttpClient> {
    client: C,
    region: Region,
    access_key_id: String,
    access_key_secret: String,
    bucket: Option<String>,
    schema: Schema,
}

impl<C: HttpClient> OSSClient<C> {
    pub fn new<'a, R, S, B, S1, S2>(
        client: C,
        region: R,
        schema: S,
        bucket: B,
        access_key_id: S1,
        access_key_secret: S2,
    ) -> Self
    where
        R: AsRef<str>,
        S: Into<Option<&'a str>>,
        B: Into<Option<&'a str>>,
        S1: Into<String>,
        S2: Into<String>,
    {
        OSSClient {
            client,
            region: region.as_ref().parse().unwrap_or_default(),
            schema: schema
                .into()
                .and_then(|_schema| _schema.parse().ok())
                .unwrap_or_default(),
            bucket: bucket.into().map(|bucket| bucket.to_string()),
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
        }
    }
    pub fn bucket(&self) -> Option<&str> {
        self.bucket.as_ref().map(String::as_str)
    }
    pub fn get_signed_url<'a, H>(
        &self,
        object: Option<&str>,
        verb: &str,
        expires: u64,
        params: &Params,
        headers: H,
    ) -> String
    where
        H: Into<Option<BTreeMap<&'a str, &'a str>>>,
    {
        let mut content_type = "";
        let mut content_md5 = "";
        let mut oss_headers_str = String::new();
        if let Some(_headers) = headers.into() {
            for (k, v) in _headers {
                if k.starts_with(OSS_CANONICALIZED_PREFIX) {
                    oss_headers_str += k;
                    oss_headers_str += ":";
                    oss_headers_str += v;
                    oss_headers_str += "\n";
                } else if k == CONTENT_TYPE {
                    content_type = v;
                } else if k == CONTENT_MD5 {
                    content_md5 = v;
                }
            }
        }
        let oss_resource_str = canonicalized_resource(self.bucket(), object, params);
        let sign_str = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            verb, content_md5, content_type, expires, oss_headers_str, oss_resource_str
        );
        let mut hasher = Hmac::new(Sha1::new(), self.access_key_secret.as_bytes());
        hasher.input(sign_str.as_bytes());
        let sign_str_base64 = encode(hasher.result().code());

        let auth_params = format!(
            "OSSAccessKeyId={}&Expires={}&Signature={}",
            self.access_key_id, expires, sign_str_base64
        );
        self.host(object, &auth_params)
    }
    fn host(&self, object: Option<&str>, params_str: &str) -> String {
        let mut host = format!("{}://", self.schema);
        if let Some(bucket) = self.bucket() {
            host.push_str(bucket);
            host.push('.');
        }
        host.push_str(self.region.endpoint());
        if let Some(object) = object {
            host.push('/');
            host.push_str(object);
        }
        host.push_str(params_str);
        host
    }
}

impl<C: HttpClient> OSSClient<C> {
    pub(crate) fn get_access_key(&self) -> (&str, &str) {
        (&self.access_key_id, &self.access_key_secret)
    }

    pub(crate) fn get_schema(&self) -> Schema {
        self.schema
    }
}
impl<C: HttpClient> OSSClient<C> {
    pub(crate) fn singed_request(&self, mut rqst: Request) -> Result<HttpRequest> {
        self.oss_sign(&mut rqst)?;
        self.generate_http_request(rqst)
    }
    pub(crate) async fn sign_and_dispatch<'a>(&self, mut rqst: Request<'a>) -> Result<Response> {
        let len = rqst.content_length();
        rqst.headers_mut()
            .insert("content-length", HeaderValue::from(len));
        let request = self.singed_request(rqst)?;
        Ok(self.client.dispatch(request).await?.into())
    }
    pub(crate) fn generate_http_request(&self, mut rqst: Request) -> Result<HttpRequest> {
        let mut url = Url::from_str(&self.host(rqst.get_object(), ""))?;
        let mut query = url.query_pairs_mut();
        for (name, value) in rqst.get_params() {
            if let Some(value) = value {
                query.append_pair(name, value);
            } else {
                query.append_key_only(name);
            }
        }
        drop(query);
        let body = rqst.take_payload().into_body();
        Ok(HttpRequest::new(
            rqst.get_method().to_owned(),
            url,
            body,
            rqst.into_headers(),
        ))
    }
}

impl OSSClient<http_client::DefaultClient> {
    pub fn new_with_default_client<'a, R, S, B, S1, S2>(
        region: R,
        schema: S,
        bucket: B,
        access_key_id: S1,
        access_key_secret: S2,
    ) -> Self
    where
        R: AsRef<str>,
        S: Into<Option<String>>,
        B: Into<Option<String>>,
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            client: http_client::default_client(),
            region: region.as_ref().parse().unwrap_or_default(),
            schema: schema
                .into()
                .and_then(|_schema| _schema.parse().ok())
                .unwrap_or_default(),
            bucket: bucket.into(),
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
        }
    }
}
