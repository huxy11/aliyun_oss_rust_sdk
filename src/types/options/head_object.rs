use headers_serializer::ToMaps;
#[derive(Clone, Debug, Default, PartialEq, ToMaps)]
// #[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HeadObjectOptions {
    /// <p>If the time specified in this header is earlier than the object modified time or does not conform to the standards, OSS returns the object and 200 OK. If the time specified in this header is later than or the same as the object modified time, OSS returns 304 Not Modified.</p>
    /// <p>The time must be in GMT. Example: Wed, 07 Oct 2020 14:47:53 GMT.</p>
    #[label("opts")]
    pub if_modified_since: Option<String>,
    /// <p>If the time specified in this header is the same as or later than the object modified time, OSS returns the object and 200 OK. If the time specified in this header is earlier than the object modified time, OSS returns 412 Precondition Failed.</p>
    /// <p>The time must be in GMT. Example: Wed, 07 Oct 2020 14:47:53 GMT.</p>
    #[label("opts")]
    pub if_unmodified_since: Option<String>,
    /// <p>If the ETag specified in the request matches the ETag value of the object, OSS transmits the object and returns 200 OK. If the ETag specified in the request does not match the ETag value of the object, OSS returns 412 Precondition Failed.</p>
    /// <p>The ETag value of an object is used to check whether the content of the object has changed. You can check data integrity by using the ETag value.</p>
    #[label("opts")]
    pub if_match: Option<String>,
    /// <p>If the ETag specified in the request does not match the ETag value of the object, OSS transmits the object and returns 200 OK. If the ETag specified in the request matches the ETag value of the object, OSS returns 304 Not Modified.</p>
    /// <p>You can specify both the If-Match and If-None-Match headers in a request.</p>
    #[label("opts")]
    pub if_none_match: Option<String>,
    // pub metas: Option<Metas>,
}
