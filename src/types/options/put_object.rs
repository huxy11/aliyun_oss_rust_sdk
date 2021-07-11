use headers_serializer::ToMaps;

use crate::types::Metas;
#[derive(Clone, Debug, Default, PartialEq, ToMaps)]
// #[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutObjectOptions {
    /// <p>The web page caching behavior that is specified when the object is downloaded.</p>
    #[label("opts")]
    pub cache_control: Option<String>,
    /// <p></p>
    #[label("opts")]
    pub content_disposition: Option<String>,
    /// <p>The content encoding type of the object during the download. </p>
    #[label("opts")]
    pub content_encoding: Option<String>,
    /// <p>The MD5 hash of the object you want to upload. The value of Content-MD5 is calculated based on the MD5 algorithm. After the Content-MD5 request header is uploaded, OSS calculates the MD5 hash of the received object and checks whether the calculated MD5 hash is the same as the Content-MD5 value provided in the request.</p>
    /// <p>To ensure data integrity, OSS provides multiple methods for you to check the MD5 hashes of the data. To perform MD5 verification based on the Content-MD5 header, add the Content-MD5 header to the request.</p>
    #[label("opts")]
    pub content_md5: Option<String>,
    /// <p>The ETag that is generated when an object is created. ETags are used to identify the content of the objects.</p>
    /// <p><li>If an object is created by using a PutObject request, the ETag value is the MD5 hash of the object content.</li>
    /// <li>If an object is created by using other methods, the ETag value is the UUID of the object content.</li></p>
    /// <p> Note: <li>The ETag value of the object can be used to check whether the object content is modified. To verify data integrity, we recommend that you do not use the ETag of an object as the MD5 hash of the object.</li></p>
    #[label("opts")]
    pub e_tag: Option<String>,
    /// <p>The time period after which the response is considered expired.</p>
    #[label("opts")]
    pub expires: Option<String>,
    /// <p>Specifies whether the PutObject operation overwrites objects of the same name. When the versioning status of the requested bucket is enabled or suspended, the x-oss-forbid-overwrite request header is invalid. In this case, the PutObject operation overwrites objects of the same name.</p>
    /// <p><li>If x-oss-forbid-overwrite is not specified or the value of x-oss-forbid-overwrite is set to false, an existing object has the same name as that of the object you want to upload can be overwritten.</li>
    /// <li>If the value of x-oss-forbid-overwrite is set to true, an existing object that has the same name as that of the object you want to upload cannot be overwritten.</li></p>
    /// <p>If you specify the x-oss-forbid-overwrite request header, the queries per second (QPS) performance of OSS may be degraded. If you want to use the x-oss-forbid-overwrite request header to perform a large number of operations (QPS greater than 1,000), submit a ticket.</p>
    #[label("opts")]
    pub x_oss_forbid_overwrite: Option<String>,
    /// <p>The server-side encryption method that is used when OSS creates the object.</p>
    /// <p>Valid values: AES256 and KMS</p>
    /// <p>If you specify this parameter, this parameter is returned in the response header and the uploaded object is encrypted and stored. When you download the encrypted object, the x-oss-server-side-encryption header is included in the response and the header value is set to the algorithm used to encrypt the object.</p>
    #[label("opts")]
    pub x_oss_server_side_encryption: Option<String>,
    /// <p>The ID of the customer master key (CMK) hosted in KMS.</p>
    /// <p>This parameter is valid only when x-oss-server-side-encryption is set to KMS.</p>
    #[label("opts")]
    pub x_oss_server_side_encryption_key_id: Option<String>,
    /// <p>The access control list (ACL) of the object you want to create.</p>
    /// <p>Valid values: public-read, private, and public-read-write</p>
    #[label("opts")]
    pub x_oss_object_acl: Option<String>,
    /// <p>The storage class of an object.</p>
    /// <p>If you specify the storage class when you upload the object, the specified storage class applies regardless of the storage class of the bucket that contains the object. If you set x-oss-storage-class to Standard when you upload an object to an IA bucket, the object is stored as a Standard object.</p>
    /// <p>Valid values: Standard, IA, Archive, and ColdArchive.</p>
    /// <p>Supported operations: PutObject, InitiateMultipartUpload, AppendObject, PutObjectSymlink, and CopyObject.</p>
    #[label("opts")]
    pub x_oss_storage_class: Option<String>,
    /// <p>The object tag. You can configure multiple tags for the object. Example: TagA=A&TagB=B.</p>
    /// <p>Note: <li>The tag key and value must be URL-encoded. If a key-value pair does not contain an equal sign (=), the tag value is considered an empty string.</li></p>
    #[label("opts")]
    pub x_oss_tagging: Option<String>,

    /// <p>If the PutObject request contains a parameter prefixed with x-oss-meta-*, the parameter is considered to be user metadata. Example: x-oss-meta-location. An object can have multiple similar parameters. However, the total size of the user metadata cannot exceed 8 KB.</p>
    /// <p>Metadata supports hyphens (-), digits, and letters. Uppercase letters are converted to lowercase letters, and other characters such as underscores (_) are not supported.</p>
    pub metas: Option<Metas>,
}
