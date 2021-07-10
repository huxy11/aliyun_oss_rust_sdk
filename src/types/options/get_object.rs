use headers_serializer::ToMaps;

use crate::types::Metas;

#[derive(Clone, Debug, Default, PartialEq, ToMaps)]
// #[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetObjectOptions {
    /// <p>The content-type header in the response that OSS returns.</p>
    #[label("opts")]
    pub response_content_type: Option<String>,
    /// <p>The content-language header in the response that OSS returns.</p>
    #[label("opts")]
    pub response_content_language: Option<String>,
    /// <p>The expires header in the response that OSS returns.</p>
    #[label("opts")]
    pub response_expires: Option<String>,
    /// <p>The cache-control header in the response that OSS returns.</p>
    #[label("opts")]
    pub response_cache_control: Option<String>,
    /// <p>The content-disposition header in the response that OSS returns.</p>
    #[label("opts")]
    pub response_content_disposition: Option<String>,
    /// <p>The content-encoding header in the response that OSS returns.</p>
    #[label("opts")]
    pub response_content_encoding: Option<String>,
    /// <p>The range of data to be returned.</p>
    /// <p><li>If the value of Range is valid, OSS returns the response that includes the total size of the object and the range of data returned. For example, Content-Range: bytes 0~9/44 indicates that the total size of the object is 44 bytes, and the range of data returned is the first 10 bytes.</li><li>However, if the value of Range is invalid, the entire object is returned, and the response returned by OSS excludes Content-Range.</li></p>
    #[label("opts")]
    pub range: Option<String>,
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
    /// <p>The encoding type at the client side.</p>
    /// <p>If you want an object to be returned in the GZIP format, you must include the Accept-Encoding:gzip header in your request. OSS determines whether to return the object compressed in the GZIP format. OSS evaluates the decision based on the Content-Type header and whether the size of the object is larger than or equal to 1 KB.</p>
    /// <p>Note
    /// <li>If an object is compressed in the GZIP format, the response OSS returns does not include the ETag value of the object.</li>
    /// <li>OSS supports the following Content-Type values to compress the object in the GZIP format: text/cache-manifest, text/xml, text/plain, text/css, application/javascript, application/x-javascript, application/rss+xml, application/json, and text/json.</li>
    /// </p>
    #[label("opts")]
    pub accept_encoding: Option<String>,
    /// <p>If the PutObject request contains a parameter prefixed with x-oss-meta-*, the parameter is considered to be user metadata. Example: x-oss-meta-location. An object can have multiple similar parameters. However, the total size of the user metadata cannot exceed 8 KB.</p>
    /// <p>Metadata supports hyphens (-), digits, and letters. Uppercase letters are converted to lowercase letters, and other characters such as underscores (_) are not supported.</p>
    pub metas: Option<Metas>,
}
