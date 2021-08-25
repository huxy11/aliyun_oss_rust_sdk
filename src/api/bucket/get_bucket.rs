use super::*;

impl<C: HttpClient> OSSClient<C> {
    pub async fn get_bucket<Opts>(&self, options: Opts) -> Result<Response>
    where
        Opts: Into<Option<GetBucketOptions>>,
    {
        let mut rqst = Request::new(Method::GET, None, None, self.get_schema(), None, None, None);
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
    use crate::api::tests::oss_client;
    use tokio::io::AsyncReadExt;
    #[tokio::test]
    async fn get_bucket_test() {
        let oss_cli = oss_client();
        let options = GetBucketOptions {
            ..Default::default()
        };
        let ret = oss_cli.get_bucket(options).await.unwrap();
        assert_eq!(ret.status.as_u16(), 200);

        let mut buf = String::new();
        ret.body
            .into_async_read()
            .read_to_string(&mut buf)
            .await
            .unwrap();
        println!("{}", buf);
    }
}
