use http_client::{HttpClient, HttpResponse};
use hyper::{header::HeaderName, Method};

use crate::{oss::OSSClient, GetObjectOptions};
use crate::{Payload, PutObjectOptions, Request, Result};

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
            self.get_schema(),
            None,
            None,
            None,
        );
        let opts = options.into().unwrap_or_default();
        rqst.add_metas(opts.metas.as_ref())?;
        for (key, val) in opts.to_opts() {
            rqst.headers_mut()
                .insert(key.parse::<HeaderName>()?, val.parse()?);
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
            self.get_schema(),
            Some(payload),
            None,
            None,
        );
        let opts = options.into().unwrap_or_default();
        rqst.add_metas(opts.metas.as_ref())?;
        for (key, val) in opts.to_opts() {
            rqst.headers_mut()
                .insert(key.parse::<HeaderName>()?, val.parse()?);
        }
        println!("rqst headers: {:?}", rqst.headers());
        self.sign_and_dispatch(rqst).await
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

    fn oss_client() -> OSSClient<http_client::DefaultClient> {
        let bucket = std::env::var("OSS_BUCKET").unwrap();
        let access_key_id = std::env::var("OSS_KEY_ID").unwrap();
        let access_key_secret = std::env::var("OSS_KEY_SECRET").unwrap();

        OSSClient::new_with_default_client(
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
