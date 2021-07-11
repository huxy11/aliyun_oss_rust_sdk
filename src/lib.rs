#[macro_use]
extern crate derive_more;
extern crate headers_serializer;
extern crate pin_project;

mod api;
mod auth;
mod oss;
mod statics;
mod types;

pub(crate) use statics::*;

pub use oss::OSSClient;
pub use types::*;
