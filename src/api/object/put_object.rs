use super::*;
impl<C: HttpClient> OSSClient<C> {
    pub async fn put_object<S, Opts>(
        &self,
        object: S,
        payload: Payload,
        options: Opts,
    ) -> Result<Response>
    where
        S: AsRef<str>,
        Opts: Into<Option<PutObjectOptions>>,
    {
        let mut rqst = Request::new(
            Method::PUT,
            self.bucket(),
            Some(object.as_ref()),
            self.get_schema(),
            Some(payload),
            None,
            None,
        );
        let options = options.into().unwrap_or_default();
        rqst.add_metas(options.metas.as_ref())?;
        for (key, val) in options.to_opts() {
            rqst.headers_mut()
                .insert(key.parse::<HeaderName>()?, val.parse()?);
        }
        self.sign_and_dispatch(rqst).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api::tests::*, Metas};
    use tokio::io::AsyncReadExt;
    #[tokio::test]
    async fn put_buffer_object_test() {
        let oss_cli = oss_client();
        let mut metas = Metas::default();
        metas.insert(META_KEY.to_owned(), META_VAL.to_owned());
        let options = PutObjectOptions {
            metas: Some(metas),
            ..Default::default()
        };
        let payload = Payload::Buffer(BUF.into());
        let ret = oss_cli
            .put_object(FILE_NAME, payload, options)
            .await
            .unwrap();
        assert_eq!(ret.status.as_u16(), 200);
        let server = ret.headers.get("server").unwrap();
        assert_eq!(server, "AliyunOSS");
        let content_length = ret.headers.get("content-length").unwrap();
        assert_eq!(content_length, "0");

        let mut buf = if let (_, Some(size_hint)) = ret.body.size_hint() {
            String::with_capacity(size_hint)
        } else {
            String::new()
        };
        let mut reader = ret.body.into_async_read();
        reader.read_to_string(&mut buf).await.unwrap();
        assert_eq!(buf.len(), 0);
    }
    #[tokio::test]
    async fn put_stream_object_test() {
        let oss_cli = oss_client();
        let mut metas = Metas::default();
        metas.insert(META_KEY.to_owned(), META_VAL.to_owned());
        let options = PutObjectOptions {
            metas: Some(metas),
            ..Default::default()
        };
        let payload = Payload::Stream(BUF.to_owned().into());
        let ret = oss_cli
            .put_object("test-with-stream", payload, options)
            .await
            .unwrap();
        assert_eq!(ret.status.as_u16(), 200);
        let server = ret.headers.get("server").unwrap();
        assert_eq!(server, "AliyunOSS");
        let content_length = ret.headers.get("content-length").unwrap();
        assert_eq!(content_length, "0");

        let mut buf = if let (_, Some(size_hint)) = ret.body.size_hint() {
            String::with_capacity(size_hint)
        } else {
            String::new()
        };
        let mut reader = ret.body.into_async_read();
        reader.read_to_string(&mut buf).await.unwrap();
        assert_eq!(buf.len(), 0);
    }
}
