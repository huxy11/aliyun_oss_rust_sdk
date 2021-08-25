use super::*;

impl<C: HttpClient> OSSClient<C> {
    /// You can call this operation to query an object. To perform the GetObject operation, you must have the read permissions on the object.
    pub async fn get_object<S, Opts>(&self, object: S, options: Opts) -> Result<Response>
    where
        S: AsRef<str>,
        Opts: Into<Option<GetObjectOptions>>,
    {
        let mut rqst = Request::new(
            Method::GET,
            self.bucket(),
            Some(object.as_ref()),
            self.get_schema(),
            None,
            None,
            None,
        );
        let options = options.into().unwrap_or_default();
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
    use crate::api::tests::*;
    use tokio::io::AsyncReadExt;
    #[tokio::test]
    async fn get_object_test() {
        let oss_cli = oss_client();

        /* Default Options */
        let options = GetObjectOptions {
            ..Default::default()
        };

        let ret = oss_cli.get_object(FILE_NAME, options).await.unwrap();
        assert_eq!(ret.status.as_u16(), 200);

        let server = ret.headers.get("server").unwrap();
        assert_eq!(server, "AliyunOSS");
        let content_length = ret.headers.get("content-length").unwrap();
        assert_eq!(content_length, "23");
        let meta_key = ret.headers.get(META_KEY_WITH_PREFIX).unwrap();
        assert_eq!(meta_key, META_VAL);

        let mut buf = if let (_, Some(size_hint)) = ret.body.size_hint() {
            String::with_capacity(size_hint)
        } else {
            String::new()
        };

        let mut reader = ret.body.into_async_read();
        reader.read_to_string(&mut buf).await.unwrap();
        assert_eq!(buf.as_bytes(), BUF);
    }
}
