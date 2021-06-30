use std::{error::Error as StdError, fmt};

use http::header::{InvalidHeaderName, InvalidHeaderValue};

use crate::HttpError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

type BoxedError = Box<dyn StdError + Send + Sync>;

pub struct Error {
    kind: Kind,
    source: Option<BoxedError>,
    // url: Option<Url>,
}
#[derive(Debug)]
pub(crate) enum Kind {
    Http,
    InvalidHeader,
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
}

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
    fn from(e: InvalidHeaderValue) -> Error {
        Error::new(Kind::InvalidHeader, e)
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
// impl From<QxmlError> for Error {
//     fn from(e: QxmlError) -> Error {
//         Error::Qxml(e)
//     }
// }
// #[derive(Debug, Display)]
// pub enum Error {
//     Object(ObjectError),
//     // Io(IoError),
//     // String(FromUtf8Error),
//     // Str(Utf8Error),
//     // HttpError(HttpError),
//     // ParseRegion(ParseRegionError),
//     // Qxml(QxmlError),
//     // ParseBool(ParseBoolError),
// }

// impl From<QxmlError> for Error {
//     fn from(e: QxmlError) -> Error {
//         Error::Qxml(e)
//     }
// }

// impl From<IoError> for Error {
//     fn from(e: IoError) -> Error {
//         Error::Io(e)
//     }
// }

// impl From<HttpError> for Error {
//     fn from(e: HttpError) -> Error {
//         Error::HttpError(e)
//     }
// }

// impl From<FromUtf8Error> for Error {
//     fn from(e: FromUtf8Error) -> Error {
//         Error::String(e)
//     }
// }
// impl From<Utf8Error> for Error {
//     fn from(e: Utf8Error) -> Error {
//         Error::Str(e)
//     }
// }

// impl From<ParseBoolError> for Error {
//     fn from(e: ParseBoolError) -> Error {
//         Error::ParseBool(e)
//     }
// }
// impl From<ParseRegionError> for Error {
//     fn from(e: ParseRegionError) -> Error {
//         Error::ParseRegion(e)
//     }
// }
// #[derive(Debug, Display)]
// pub enum ObjectError {
//     #[display(fmt = "PUT ERROR: {:#?}", msg)]
//     PutError { msg: String },
//     #[display(fmt = "GET ERROR: {:#?}", msg)]
//     GetError { msg: String },
//     #[display(fmt = "DELETE ERROR: {:#?}", msg)]
//     DeleteError { msg: String },
//     #[display(fmt = "HEAD ERROR: {:#?}", msg)]
//     HeadError { msg: String },
// }

// pub type Result<T> = std::result::Result<T, Error>;

// impl StdError for Error {}
