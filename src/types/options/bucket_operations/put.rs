use headers_serializer::ToMaps;

#[derive(Clone, Debug, Default, PartialEq, ToMaps)]
// #[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutBucketOptions {
    /// <p>The access control list (ACL) of the bucket that you want to create.</p>
    /// <p> Valid values:
    /// <li>public-read-write</li>
    /// <li>public-read</li>
    /// <li>private</li></p>
    #[label("opts")]
    pub x_oss_acl: Option<String>,
    /// <p>Specifies whether to enable the hierarchical namespace feature for the bucket you want to create.</p>
    /// <p>Default value: disabled.</p>
    /// <p>You can enable or disable the hierarchical namespace feature for a bucket only when you create the bucket. The hierarchical namespace feature cannot be enabled or disabled for an existing bucket.</p>
    /// <p><li>enabled: The hierarchical namespace feature is enabled for the bucket.After the hierarchical namespace feature is enabled for the bucket, you can create, delete, and rename directories.</li>
    /// <li>disabled: The hierarchical namespace feature is disabled for the bucket.</li></p>
    #[label("opts")]
    pub x_oss_hns_status: Option<String>,
}
