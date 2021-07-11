use oss_sdk::{Metas, OSSClient, Payload, PutObjectOptions};
use tokio::io::AsyncReadExt;

const BUF: &str = "Test Content";
const FILE_NAME: &str = "test_file";
const META_KEY: &str = "test-meta-key";
const META_KEY_WITH_PREFIX: &str = "x-oss-meta-test-meta-key";
const META_VAL: &str = "test-meta-val";

#[tokio::test]
async fn example() {
    let bucket = std::env::var("OSS_BUCKET").unwrap();
    let access_key_id = std::env::var("OSS_KEY_ID").unwrap();
    let access_key_secret = std::env::var("OSS_KEY_SECRET").unwrap();

    let oss_cli = OSSClient::new_with_default_client(
        "北京",
        None,
        bucket,
        access_key_id.to_owned(),
        access_key_secret.to_owned(),
    );
    /* Put Object */
    // With Metas
    let mut metas = Metas::default();
    metas.insert(META_KEY.to_owned(), META_VAL.to_owned());
    let opts = PutObjectOptions {
        metas: Some(metas),
        ..Default::default()
    };
    let payload = Payload::Stream(BUF.as_bytes().to_owned().into());
    let ret = oss_cli.put_object(FILE_NAME, payload, opts).await;
    assert!(ret.is_ok());

    /* Get Object*/
    // Default Options
    let ret = oss_cli.get_object(FILE_NAME, None).await.unwrap();
    assert!(ret.status.is_success());
    assert!(ret.headers.contains_key(META_KEY_WITH_PREFIX));
    let mut reader = ret.body.into_async_read();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).await.unwrap();
    assert_eq!(BUF, buf);
}
