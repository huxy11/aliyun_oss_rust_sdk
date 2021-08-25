use super::*;

impl<C: HttpClient> OSSClient<C> {
    pub async fn put_bucket<S, Opts>(&self, bucket: S, options: Opts) -> Result<Response>
    where
        S: AsRef<str>,
        Opts: Into<Option<PutBucketOptions>>,
    {
        let mut rqst = Request::new(
            Method::PUT,
            Some(bucket.as_ref()),
            None,
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
    use crate::api::tests::oss_client;
    use tokio::io::AsyncReadExt;

    /* Bucket Operations */
    #[tokio::test]
    async fn put_bucket_test() {
        let oss_cli = oss_client();
        let options = PutBucketOptions {
            ..Default::default()
        };
        let bucket = oss_cli.bucket().unwrap().to_owned() + "oss-sdk-test";
        let ret = oss_cli.put_bucket(bucket, options).await.unwrap();
        // assert_eq!(ret.status.as_u16(), 200);
        let mut buf = String::new();
        ret.body
            .into_async_read()
            .read_to_string(&mut buf)
            .await
            .unwrap();
        println!("{}", buf);
    }
}
