#[macro_use]
extern crate derive_more;
extern crate headers_serializer;
extern crate pin_project;

mod auth;
mod oss;
mod statics;
mod types;

pub use crate::oss::OSSClient;
pub use statics::*;
pub use types::*;
