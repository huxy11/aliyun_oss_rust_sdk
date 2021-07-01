use super::*;
use std::collections::BTreeMap;

mod auth;
mod default_client;
mod errors;
mod responses;
mod sign_and_dispatch;
mod signed_requests;
mod stream;

pub use errors::HttpError;
pub use responses::HttpResponse;
pub use sign_and_dispatch::SignAndDispatch;
pub use signed_requests::SignedRequest;

type Params = BTreeMap<String, Option<String>>;
// type Headers = BTreeMap<String, String>;
