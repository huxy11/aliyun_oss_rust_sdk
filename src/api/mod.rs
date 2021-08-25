mod bucket;
mod object;

use http_client::HttpClient;
use hyper::{header::HeaderName, Method};

use crate::{oss::OSSClient, GetObjectOptions};
use crate::{
    GetBucketOptions, HeadObjectOptions, Payload, PutBucketOptions, PutObjectOptions, Request,
    Response, Result,
};

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const BUF: &[u8] = "This is just a put test".as_bytes();
    pub(crate) const FILE_NAME: &str = "test-with-header";
    pub(crate) const META_KEY: &str = "test-meta-key";
    pub(crate) const META_KEY_WITH_PREFIX: &str = "x-oss-meta-test-meta-key";
    pub(crate) const META_VAL: &str = "test-meta-val";

    pub(crate) fn oss_client() -> OSSClient<http_client::DefaultClient> {
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
}
