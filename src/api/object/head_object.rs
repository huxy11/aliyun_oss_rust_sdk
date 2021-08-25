use super::*;
impl<C: HttpClient> OSSClient<C> {
    pub async fn head_object<S, Opts>(&self, object: S, options: Opts) -> Result<Response>
    where
        S: AsRef<str>,
        Opts: Into<Option<HeadObjectOptions>>,
    {
        let mut rqst = Request::new(
            Method::HEAD,
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
    use crate::api::tests::*;
    #[tokio::test]
    async fn head_object_test() {
        let oss_cli = oss_client();
        let ret = oss_cli.head_object(FILE_NAME, None).await.unwrap();
        assert_eq!(ret.status.as_u16(), 200);
        let server = ret.headers.get("server").unwrap();
        assert_eq!(server, "AliyunOSS");
        let content_length = ret.headers.get("content-length").unwrap();
        assert_eq!(content_length, "23");
        let meta_key = ret.headers.get(META_KEY_WITH_PREFIX).unwrap();
        assert_eq!(meta_key, META_VAL);
    }
}
