## About
Alibaba Cloud Object Storage Service (OSS) is a cloud storage service provided by Alibaba Cloud, featuring massive capacity, security, a low cost, and high reliability. You can upload and download data on any application anytime and anywhere by calling APIs, and perform simple management of data through the web console. The OSS can store any type of files and therefore applies to various websites, development enterprises and developers. The OSS SDK for Rust provides a variety of interfaces for convenient use of the OSS. 

## Installation

To use OSS SDK in your Rust program built with Cargo, add it as a dependency as below:
```toml
[dependencies]
oss_sdk = {version = "0.3", git="https://github.com/huxy11/oss_sdk}
```

## Usage

All public types are reexported to the crate root. Consult the rustdoc documentation for full details by running ```cargo doc```.

A simple example of using:
```Rust
		const BUF: &str = "Test Content";
		const FILE_NAME: &str = "test_file";
    	const META_KEY: &str = "test-meta-key";
    	const META_KEY_WITH_PREFIX: &str = "x-oss-meta-test-meta-key";
    	const META_VAL: &str = "test-meta-val";

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
        metas.insert("test-meta-key".to_owned(), "test-meta-val".to_owned());
        let opts = PutObjectOptions {
            metas: Some(metas),
            ..Default::default()
        };
        let payload = Payload::Stream(BUF.to_owned().into());
        let ret = oss_cli
            .put_object("test-with-stream", payload, opts)
            .await;
		assert_eq!(ret.is_ok());

		/* Get Object*/
		// Default Options
        let ret = oss_cli.get_object(FILE_NAME, None).await.unwrap();
		assert!(ret.status.is_success());


		

```