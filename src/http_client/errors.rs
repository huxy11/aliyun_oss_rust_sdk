use std::{error::Error as StdError, fmt};

use http::StatusCode;
use url::Url;

pub(crate) type HttpResult<T> = Result<T, HttpError>;

type BoxedError = Box<dyn StdError + Send + Sync>;

pub struct HttpError {
    kind: Kind,
    source: Option<BoxedError>,
    url: Option<Url>,
}

impl HttpError {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> HttpError
    where
        E: Into<BoxedError>,
    {
        Self {
            kind,
            source: source.map(Into::into),
            url: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Kind {
    Body,
    Client,
    Status(StatusCode),
    Header,
    Url,
    Method,
}

pub(crate) fn url<E: Into<BoxedError>>(e: E) -> HttpError {
    HttpError::new(Kind::Url, Some(e))
}
pub(crate) fn body<E: Into<BoxedError>>(e: E) -> HttpError {
    HttpError::new(Kind::Body, Some(e))
}
pub(crate) fn header<E: Into<BoxedError>>(e: E) -> HttpError {
    HttpError::new(Kind::Header, Some(e))
}
pub(crate) fn method<E: Into<BoxedError>>(e: E) -> HttpError {
    HttpError::new(Kind::Method, Some(e))
}
pub(crate) fn client<E: Into<BoxedError>>(e: E) -> HttpError {
    HttpError::new(Kind::Client, Some(e))
}

impl fmt::Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("reqwest::Error");

        builder.field("kind", &self.kind);

        if let Some(ref url) = self.url {
            builder.field("url", url);
        }
        if let Some(ref source) = self.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}
impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct ForUrl<'a>(Option<&'a Url>);

        impl fmt::Display for ForUrl<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if let Some(url) = self.0 {
                    write!(f, " for url ({})", url.as_str())
                } else {
                    Ok(())
                }
            }
        }

        match self.kind {
            Kind::Body => f.write_str("request or response body error")?,
            Kind::Status(ref code) => {
                let prefix = if code.is_client_error() {
                    "HTTP status client error"
                } else {
                    debug_assert!(code.is_server_error());
                    "HTTP status server error"
                };
                write!(f, "{} ({})", prefix, code)?;
            }
            _ => unimplemented!(),
        };

        ForUrl(self.url.as_ref()).fmt(f)?;

        if let Some(ref e) = self.source {
            write!(f, ": {}", e)?;
        }

        Ok(())
    }
}

impl StdError for HttpError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

// #[derive(Debug, PartialEq, Display)]
// pub enum DispatchError {
// InvalidMethod,
// HeaderError(String),
// InternalError(String),
// Unknown(String),
// /// A service-specific error occurred.
// Service(E),
// /// An error occurred dispatching the HTTP request
// HttpDispatch(HttpDispatchError),
// /// An error was encountered with AWS credentials.
// Credentials(CredentialsError),
// /// A validation error occurred.  Details from AWS are provided.
// Validation(String),
// /// An error occurred parsing the response payload.
// ParseError(String),
// /// An unknown error occurred.  The raw HTTP response is provided.
// Unknown(BufferedHttpResponse),
// /// An error occurred when attempting to run a future as blocking
// Blocking,
// }

// impl From<reqwest::Error> for DispatchError {
//     fn from(e: reqwest::Error) -> Self {
//         Self::InternalError(e.to_string())
//     }
// }
// impl From<reqwest::header::InvalidHeaderName> for DispatchError {
//     fn from(e: reqwest::header::InvalidHeaderName) -> Self {
//         let mut s = "InvalidKey".to_string();
//         s.push_str(&e.to_string());
//         Self::HeaderError(s)
//     }
// }
// impl From<reqwest::header::InvalidHeaderValue> for DispatchError {
//     fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
//         let mut s = "InvalidValue".to_string();
//         s.push_str(&e.to_string());
//         Self::HeaderError(s)
//     }
// }

// impl std::error::Error for HttpError {}
