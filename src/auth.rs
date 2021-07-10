use base64::encode;
use chrono::Utc;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use http_client::{HttpClient, Params};
use hyper::header::{HeaderName, HeaderValue};

use crate::{statics::OSS_CANONICALIZED_PREFIX, types::*, OSSClient};

const RESOURCES: [&str; 51] = [
    "acl",
    "uploads",
    "location",
    "cors",
    "logging",
    "website",
    "referer",
    "lifecycle",
    "delete",
    "append",
    "tagging",
    "objectMeta",
    "uploadId",
    "partNumber",
    "security-token",
    "position",
    "img",
    "style",
    "styleName",
    "replication",
    "replicationProgress",
    "replicationLocation",
    "cname",
    "bucketInfo",
    "comp",
    "qos",
    "live",
    "status",
    "vod",
    "startTime",
    "endTime",
    "symlink",
    "x-oss-process",
    "response-content-type",
    "response-content-language",
    "response-expires",
    "response-cache-control",
    "response-content-disposition",
    "response-content-encoding",
    "udf",
    "udfName",
    "udfImage",
    "udfId",
    "udfImageDesc",
    "udfApplication",
    "comp",
    "udfApplicationLog",
    "restore",
    "callback",
    "callback-var",
    "continuation-token",
];

impl<C: HttpClient> OSSClient<C> {
    pub(crate) fn oss_sign(&self, rqst: &mut Request) -> Result<()> {
        let headers = rqst.headers_mut();
        headers.insert(
            HeaderName::from_static("date"),
            HeaderValue::from_str(&Utc::now().format("%a, %d %b %Y %T GMT").to_string())?,
        );
        self.add_authorization_header(rqst)
    }
    fn add_authorization_header(&self, rqst: &mut Request) -> Result<()> {
        let headers = rqst.headers();
        let date = headers
            .get("date")
            .and_then(|val| val.to_str().ok())
            .unwrap_or_default();
        let content_type = headers
            .get("content-type")
            .and_then(|val| val.to_str().ok())
            .unwrap_or_default();

        let content_md5 = headers
            .get("Content-MD5")
            .map(|val| encode(val.to_str().ok().unwrap_or_default()))
            .unwrap_or_default();

        let mut oss_headers_str = String::new();
        for (k, v) in headers.iter().filter(|(k, _)| {
            k.as_str().contains(OSS_CANONICALIZED_PREFIX)
            // && !k.as_str().contains(OSS_META_PREFIX)
        }) {
            oss_headers_str += &format!(
                "{}:{}\n",
                k,
                v.to_str().map_err(Error::header_to_str_error)?
            );
        }

        let oss_resource_str =
            canonicalized_resource(self.get_bucket(), rqst.get_object(), rqst.get_params());
        let sign_str = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            rqst.get_method(),
            content_md5,
            content_type,
            date,
            oss_headers_str,
            oss_resource_str
        );

        let (access_key_id, access_key_secret) = self.get_access_key();
        let mut hasher = Hmac::new(Sha1::new(), access_key_secret.as_bytes());
        hasher.input(sign_str.as_bytes());
        let sign_str_base64 = encode(hasher.result().code());

        let authorization =
            HeaderValue::from_str(&format!("OSS {}:{}", access_key_id, sign_str_base64))?;
        rqst.headers_mut()
            .insert(HeaderName::from_static("authorization"), authorization);
        Ok(())
    }
}

#[inline]
pub(crate) fn canonicalized_resource(
    bucket: Option<&str>,
    object: Option<&str>,
    params: &Params,
) -> String {
    /*
    1. CanonicalizedResource = "/BucketName/ObjectName", "/BucketName" or "/""  + "?" + SubResources
    2. SubResources, 将所有的子资源按照字典序，从小到大排列并以&为分隔符生成子资源字符串。
     */
    
    let mut ret = String::new();
    if let Some(bucket) = bucket {
        ret.push('/');
        ret.push_str(bucket);
    }
    if let Some(object) = object {
        ret.push('/');
        ret.push_str(object);
    }
    if ret.is_empty() {
        ret.push('/');
    }
    ret.push_str(&get_resources_str(params));
    ret
}

#[inline]
fn get_resources_str(params: &Params) -> String {
    let mut resources: Vec<(String, Option<String>)> = params
        .iter()
        .filter(|(k, _)| RESOURCES.contains(&k.as_str()))
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();
    // TODO Delete this line
    resources.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = String::new();
    for (k, v) in resources {
        if result.is_empty() {
            result += "?";
        } else {
            result += "&";
        }
        if let Some(vv) = v {
            result += &format!("{}={}", k, vv);
        } else {
            result += &k;
        }
    }
    result
}
