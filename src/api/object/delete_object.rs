use super::*;

impl<C: HttpClient> OSSClient<C> {
    pub async fn delete_object<S>(&self, object: S) -> Result<Response>
    where
        S: AsRef<str>,
    {
        let rqst = Request::new(
            Method::DELETE,
            self.bucket(),
            Some(object.as_ref()),
            self.get_schema(),
            None,
            None,
            None,
        );
        self.sign_and_dispatch(rqst).await
    }
}
#[cfg(test)]
mod tests {
    use crate::api::tests::*;
    #[tokio::test]
    async fn delete_object_test() {
        let oss_cli = oss_client();
        let ret = oss_cli
            .delete_object(FILE_NAME.to_owned() + "-sec")
            .await
            .unwrap();
        // HTTP status code 204 is returned when the DeleteObject operation succeeds, regardless of whether the object exists.
        assert_eq!(ret.status.as_u16(), 204);
    }
}
