use base64::encode;
use crypto::{hmac::Hmac, mac::Mac, sha1::Sha1};
use http_client::{HttpClient, HttpRequest, HttpResponse, Params};
use hyper::{
    header::{HeaderName, HeaderValue},
    Method,
};
use url::Url;

use std::{collections::BTreeMap, str::FromStr};

use crate::{
    auth::canonicalized_resource,
    statics::{reqwest_client, CONTENT_MD5, CONTENT_TYPE, OSS_CANONICALIZED_PREFIX},
    types::{GetObjectOptions, Payload, PutObjectOptions, Region, Request, Result, Schema},
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
    pub fn get_bucket(&self) -> Option<&str> {
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
        let oss_resource_str = canonicalized_resource(self.get_bucket(), object, params);
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
        if let Some(bucket) = self.get_bucket() {
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
}
impl<C: HttpClient> OSSClient<C> {
    pub async fn get_object<S, Opts>(&self, object: S, options: Opts) -> Result<HttpResponse>
    where
        S: AsRef<str>,
        Opts: Into<Option<GetObjectOptions>>,
    {
        let mut rqst = Request::new(
            Method::GET,
            self.get_bucket(),
            Some(object.as_ref()),
            self.schema,
            None,
            None,
            None,
        );
        let opts = options.into().unwrap_or_default();
        rqst.add_metas(opts.metas.as_ref())?;
        for (key, val) in opts.to_opts() {
            rqst.headers_mut()
                .insert(HeaderName::from_str(&key)?, val.parse()?);
        }
        self.sign_and_dispatch(rqst).await
    }
    pub async fn put_object<S, Opts>(
        &self,
        object: S,
        payload: Payload,
        options: Opts,
    ) -> Result<HttpResponse>
    where
        S: AsRef<str>,
        Opts: Into<Option<PutObjectOptions>>,
    {
        let mut rqst = Request::new(
            Method::PUT,
            self.get_bucket(),
            Some(object.as_ref()),
            self.schema,
            Some(payload),
            None,
            None,
        );
        let opts = options.into().unwrap_or_default();
        rqst.add_metas(opts.metas.as_ref())?;
        for (key, val) in opts.to_opts() {
            rqst.headers_mut()
                .insert(HeaderName::from_str(&key)?, val.parse()?);
        }
        println!("rqst headers: {:?}", rqst.headers());
        self.sign_and_dispatch(rqst).await
    }
    // pub fn get_request<'a, S>(&self, object: S) -> HttpRequest
    // where
    //     S: Into<Option<&'a str>>,
    // {
    //     self.generate_request(Method::GET, object.into().unwrap_or_default(), None)
    // }
    //     pub fn put_request<'a, S>(&self, object: S, payload: Payload) -> SignedRequest
    //     where
    //         S: Into<String>,
    //     {
    //         self.generate_request(Method::PUT, object, Some(payload))
    //     }
    //     pub fn head_request<S>(&self, object: S) -> SignedRequest
    //     where
    //         S: Into<String>,
    //     {
    //         self.generate_request(Method::HEAD, object, None)
    //     }
    //     pub fn del_request<S>(&self, object: S) -> SignedRequest
    //     where
    //         S: Into<String>,
    //     {
    //         self.generate_request(Method::DELETE, object, None)
    //     }
    //     pub async fn sign_and_dispatch<SR>(&self, request: SR) -> Result<HttpResponse>
    //     where
    //         SR: Into<SignedRequest>,
    //     {
    //         let mut request = request.into();
    //         request.region = Some(self.region);
    //         request.access_key_id = self.access_key_id.to_owned();
    //         request.access_key_secret = self.access_key_secret.to_owned();

    //         self.client
    //             .sign_and_dispatch(request, None)
    //             .await
    //             .map_err(Error::from)
    //     }
    //     fn generate_request<'a, S1>(
    //         &self,
    //         method: Method,
    //         object: S1,
    //         payload: Option<SignedRequestPayload>,
    //     ) -> SignedRequest
    //     where
    //         S1: Into<String>,
    //     {
    //         let mut signed_rqst = SignedRequest::new(
    //             method,
    //             // &self.region,
    //             &self.bucket,
    //             object,
    //             &self.access_key_id,
    //             &self.access_key_secret,
    //             self.schema,
    //         );
    //         if let Some(_payload) = payload {
    //             signed_rqst.set_payload(_payload);
    //         }
    //         signed_rqst
    //     }
    pub(crate) fn singed_request(&self, mut rqst: Request) -> Result<HttpRequest> {
        self.oss_sign(&mut rqst)?;
        self.generate_http_request(rqst)
    }
    pub(crate) async fn sign_and_dispatch<'a>(
        &self,
        mut rqst: Request<'a>,
    ) -> Result<HttpResponse> {
        let len = rqst.content_length();
        rqst.headers_mut()
            .insert("content-length", HeaderValue::from(len));
        let request = self.singed_request(rqst)?;
        Ok(self.client.dispatch(request).await?)
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
        // Ok(HttpRequest {
        //     url: url,
        //     method: rqst.get_method().to_owned(),
        //     body: body,
        //     headers: rqst.into_headers(),
        // })
    }
}

// impl SignedRequest {
//     pub fn add_meta<'a>(
//         &mut self,
//         meta: impl IntoIterator<Item = (&'a str, &'a str)>,
//     ) -> Result<()> {
//         for (k, v) in meta {
//             let key = HeaderName::from_str(Self::add_oss_meta_prefix(k).as_ref())?;
//             let value = HeaderValue::from_str(Self::add_oss_meta_prefix(v).as_ref())?;
//             self.add_header(key, value);
//         } // self.add_headers(meta)
//         Ok(())
//     }
//     fn add_oss_meta_prefix(s: &str) -> Cow<str> {
//         if !s.starts_with(OSS_META_PREFIX) {
//             Cow::from(format!("{}{}", OSS_META_PREFIX, s))
//         } else {
//             Cow::Borrowed(&*s)
//         }
//     }
// }

// #[inline]
// fn get_oss_subresource_signed_str(bucket: &str, object: &str, oss_resources: &str) -> String {
//     let oss_resources = if oss_resources != "" {
//         String::from("?") + oss_resources
//     } else {
//         String::new()
//     };
//     if bucket == "" {
//         format!("/{}{}", bucket, oss_resources)
//     } else {
//         format!("/{}/{}{}", bucket, object, oss_resources)
//     }
// }

impl OSSClient<reqwest::Client> {
    pub fn new_with_reqwest<'a, R, S, B, S1, S2>(
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
            client: reqwest_client(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Metas;
    use futures::{AsyncReadExt, TryStreamExt};
    const BUF: &[u8] = "This is just a put test".as_bytes();

    #[tokio::test]
    async fn get_object_test() {
        let oss_cli = oss_client();

        /* Default Options */
        let opts = GetObjectOptions {
            ..Default::default()
        };

        let ret = oss_cli.get_object("test-with-stream", opts).await.unwrap();
        println!("StatusCode: {}", ret.status.to_string());
        println!("headers: {:?}", ret.headers);

        let mut reader = ret
            .body
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
            .into_async_read();
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        println!("Body: {}", buf);
    }

    #[tokio::test]
    async fn put_buffer_object_test() {
        let oss_cli = oss_client();
        let mut metas = Metas::default();
        metas.insert("test-meta-key".to_owned(), "test-meta-val".to_owned());
        let opts = PutObjectOptions {
            metas: Some(metas),
            ..Default::default()
        };
        let payload = Payload::Buffer(BUF.into());
        let ret = oss_cli
            .put_object("test-with-header", payload, opts)
            .await
            .unwrap();
        println!("StatusCode: {}", ret.status.to_string());
        println!("headers: {:?}", ret.headers);

        let mut reader = ret
            .body
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
            .into_async_read();
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        println!("Body: {}", buf);
    }
    #[tokio::test]
    async fn put_stream_object_test() {
        let oss_cli = oss_client();
        let mut metas = Metas::default();
        metas.insert("test-meta-key".to_owned(), "test-meta-val".to_owned());
        let opts = PutObjectOptions {
            metas: Some(metas),
            ..Default::default()
        };
        let payload = Payload::Stream(BUF.to_owned().into());
        let ret = oss_cli
            .put_object("test-with-stream", payload, opts)
            .await
            .unwrap();
        println!("StatusCode: {}", ret.status.to_string());
        println!("headers: {:?}", ret.headers);

        let mut reader = ret
            .body
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
            .into_async_read();
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        println!("Body: {}", buf);
    }

    fn oss_client() -> OSSClient<reqwest::Client> {
        let bucket = std::env::var("OSS_BUCKET").unwrap();
        let access_key_id = std::env::var("OSS_KEY_ID").unwrap();
        let access_key_secret = std::env::var("OSS_KEY_SECRET").unwrap();

        OSSClient::new_with_reqwest(
            "北京",
            None,
            bucket,
            access_key_id.to_owned(),
            access_key_secret.to_owned(),
        )
    }

    // #[tokio::test]
    // async fn smoke_test() {
    //     let bucket = std::env::var("OSS_BUCKET").unwrap();
    //     let access_key_id = std::env::var("OSS_KEY_ID").unwrap();
    //     let access_key_secret = std::env::var("OSS_KEY_SECRET").unwrap();

    //     let mut str_buffer = String::new();

    //     let oss_ins = OSSClient::new_with_reqwest(
    //         "北京",
    //         None,
    //         "oss_put_bucket_test",
    //         access_key_id.to_owned(),
    //         access_key_secret.to_owned(),
    //     );

    //     let rqst = PutBucketRequest {
    //         ..Default::default()
    //     };
    //     let ret = oss_ins.sign_and_dispatch(rqst).await;
    //     println!("{:?}", ret);

    //     let oss_instance = OSSClient::new_with_reqwest(
    //         "北京",
    //         None,
    //         bucket.as_ref(),
    //         access_key_id,
    //         access_key_secret,
    //     );

    //     /* Put Object  */
    //     // let payload = SignedRequestPayload::Buffer(Bytes::from(BUF));

    //     let chunk = vec![Ok(Bytes::from_static(BUF))];
    //     let stream = ByteStream::new(stream::iter(chunk));
    //     let payload = SignedRequestPayload::Stream(stream);

    //     let mut rqst = oss_instance.put_request(FILE_NAME, payload);
    //     rqst.add_meta([("test-key", "test-val")].iter().map(|a| a.to_owned()))
    //         .unwrap();
    //     let ret = oss_instance.sign_and_dispatch(rqst).await;
    //     println!("Put object ret = {:?}", ret);
    //     assert!(ret.is_ok() && ret.unwrap().status.is_success());

    //     /* Get Object */
    //     let mut rqst = oss_instance.get_request(None);
    //     rqst.add_params("prefix", "rust_oss_sdk");
    //     let ret = oss_instance.sign_and_dispatch(rqst).await.unwrap();
    //     assert!(ret.status.is_success());

    //     /* Get Object */
    //     let rqst = oss_instance.get_request(FILE_NAME);
    //     let ret = oss_instance.sign_and_dispatch(rqst).await.unwrap();
    //     ret.body
    //         .into_async_read()
    //         .read_to_string(&mut str_buffer)
    //         .await
    //         .unwrap();
    //     assert_eq!(str_buffer.as_bytes(), BUF);

    //     /* Add Header to Object */
    //     let rqst = oss_instance.head_request(FILE_NAME);
    //     let ret = oss_instance.sign_and_dispatch(rqst).await;
    //     assert!(ret.is_ok() && ret.unwrap().headers.contains_key("x-oss-meta-test-key"));

    //     /* Del Object */
    //     let rqst = oss_instance.del_request(FILE_NAME);
    //     let ret = oss_instance.sign_and_dispatch(rqst).await;
    //     assert!(ret.is_ok() && ret.unwrap().status.is_success());

    //     /* Check if del succeed */
    //     let rqst = oss_instance.get_request(FILE_NAME);
    //     let ret = oss_instance.sign_and_dispatch(rqst).await;
    //     assert!(ret.is_ok() && ret.unwrap().status.is_client_error());
    // }
}
