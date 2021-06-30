use once_cell::sync::OnceCell;
pub const OSS_PREFIX: &str = "x-oss-meta-";
pub const OSS_CANONICALIZED_PREFIX: &str = "x-oss-";

pub(crate) const CONTENT_TYPE: &str = "content-type";
pub(crate) const CONTENT_MD5: &str = "Content-MD5";

// Reusable Lazy Initialized Global reqwest::Client
static REQWEST_CLIENT: OnceCell<reqwest::Client> = OnceCell::new();
pub(crate) fn reqwest_client() -> reqwest::Client {
    REQWEST_CLIENT.get_or_init(reqwest::Client::new).clone()
}
