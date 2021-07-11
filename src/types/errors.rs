use std::{error::Error as StdError, fmt};

use http_client::HttpError;

use hyper::header::{InvalidHeaderName, InvalidHeaderValue};
use url::ParseError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

type BoxedError = Box<dyn StdError + Send + Sync>;

/// Error 类型
pub struct Error {
    kind: Kind,
    source: Option<BoxedError>,
}
#[derive(Debug)]
pub(crate) enum Kind {
    Http,
    HeaderToStrError,
    InvalidHeader,
    IoError,
    UrlParsingError,
}
impl Error {
    pub(crate) fn new<E>(kind: Kind, err: E) -> Self
    where
        E: Into<BoxedError>,
    {
        Self {
            kind,
            source: Some(err.into()),
        }
    }
    pub(crate) fn header_to_str_error<E>(err: E) -> Self
    where
        E: Into<BoxedError>,
    {
        Self::new(Kind::HeaderToStrError, err)
    }
}

/* From Traits */
impl From<HttpError> for Error {
    fn from(e: HttpError) -> Error {
        Error::new(Kind::Http, e)
    }
}
impl From<InvalidHeaderName> for Error {
    fn from(e: InvalidHeaderName) -> Error {
        Error::new(Kind::InvalidHeader, e)
    }
}
impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Error {
        Error::new(Kind::InvalidHeader, err)
    }
}
impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::new(Kind::UrlParsingError, err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new(Kind::IoError, err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("oss_sdk::Error");
        builder.field("kind", &self.kind);
        if let Some(ref source) = self.source {
            builder.field("source", source);
        }
        builder.finish()
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::Http => f.write_str("request or response body error")?,
            _ => unimplemented!(),
        };
        if let Some(ref e) = self.source {
            write!(f, ": {}", e)?;
        }
        Ok(())
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}
