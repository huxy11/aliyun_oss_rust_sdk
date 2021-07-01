use http::{header::HeaderName, HeaderValue, Method};
// use xml::{writer::XmlEvent, EventWriter};

use crate::http_client::SignedRequest;

/// You can call this operation to create a bucket.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutBucketRequest {
    // /// <p>The name of the bucket to create.</p>
    pub bucket: String,
    /// <p>The access control list (ACL) of the bucket that you want to create.</p>
    /// <p>Valid values:</p>
    /// <p><li>public-read-write</li>
    /// <li>public-read</li>
    /// <li>private</li></p>
    pub acl: Option<String>,
    /// <p>Specifies whether to enable the hierarchical namespace feature for the bucket you want to create. Default value: disabled. </p>
    /// <p>You can enable or disable the hierarchical namespace feature for a bucket only when you create the bucket. The hierarchical namespace feature cannot be enabled or disabled for an existing bucket.</p>
    /// <p><li> enabled: The hierarchical namespace feature is enabled for the bucket.After the hierarchical namespace feature is enabled for the bucket, you can create, delete, and rename directories.</li>
    /// <li> disabled: The hierarchical namespace feature is disabled for the bucket.</li></p>
    pub status: Option<String>,
    /// <p>The storage class of the bucket.<p>
    /// <p>Valid values:</p>
    /// <p><li>Standard</li>
    /// <li>IA</li>
    /// <li>Archive</li>
    /// <li>ColdArchive</li></p>
    pub storage_class: Option<String>,
    /// <p>The disaster recovery type of a bucket. Default value: LRS.</p>
    /// <p>Valid values:</p>
    /// <p><li>LRS, OSS stores the copies of each object across different devices within the same zone. This way, OSS ensures data reliability and availability when hardware failures occur.</li>
    /// <li>ZRS, Zone-redundant storage (ZRS) uses the multi-zone mechanism to distribute user data across three zones within the same region. Even if a zone becomes unavailable due to unexpected events such as power outages and fires, the data can still be accessed.</li></p>
    pub data_redundancy_type: Option<String>,
    // /// <p>The configuration information for the bucket.</p>
    // pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    // /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>
    // pub grant_full_control: Option<String>,
    // /// <p>Allows grantee to list the objects in the bucket.</p>
    // pub grant_read: Option<String>,
    // /// <p>Allows grantee to read the bucket ACL.</p>
    // pub grant_read_acp: Option<String>,
    // /// <p>Allows grantee to create new objects in the bucket.</p> <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p>
    // pub grant_write: Option<String>,
    // /// <p>Allows grantee to write the ACL for the applicable bucket.</p>
    // pub grant_write_acp: Option<String>,
    // /// <p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p>
    // pub object_lock_enabled_for_bucket: Option<bool>,
}

impl From<PutBucketRequest> for SignedRequest {
    fn from(mut rqst: PutBucketRequest) -> Self {
        let mut signed_rqst = SignedRequest {
            method: Method::PUT,
            region: None,
            bucket: rqst.bucket,
            ..Default::default()
        };
        if let Some(_val) = rqst.acl.take() {
            if let Ok(__val) = HeaderValue::from_str(&_val) {
                signed_rqst.add_header(HeaderName::from_static("x-oss-acl"), __val);
            }
        }
        if let Some(_val) = rqst.status.take() {
            if let Ok(__val) = HeaderValue::from_str(&_val) {
                signed_rqst.add_header(HeaderName::from_static("x-oss-hns-status"), __val);
            }
        }
        // let storage_class = rqst.storage_class.unwrap_or("Standard".to_owned());
        // let mut writer = EventWriter::new(Vec::new());
        // let event = XmlEvent::start_element("CreateBucketConfiguration");
        // writer.write(event);

        signed_rqst.set_content_length(0);
        signed_rqst
    }
}
