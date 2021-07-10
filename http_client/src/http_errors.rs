use std::{error::Error as StdError, fmt};

use hyper::StatusCode;
use url::Url;

pub(crate) type HttpResult<T> = Result<T, HttpError>;

type BoxedError = Box<dyn StdError + Send + Sync>;

pub struct HttpError {
    kind: Kind,
    source: Option<BoxedError>,
    url: Option<Url>,
}

impl HttpError {
    pub(crate) fn new<E>(kind: Kind, source: E) -> HttpError
    where
        E: Into<BoxedError>,
    {
        Self {
            kind,
            source: Some(source.into()),
            url: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Kind {
    Request,
    Status(StatusCode),
    TimedOut,
    Dispatch,
}

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        let kind = if e.is_status() {
            Kind::Status(
                e.status()
                    .expect("Status Error Occurred But No Status Code."),
            )
        } else if e.is_timeout() {
            Kind::TimedOut
        } else if e.is_request() {
            Kind::Request
        } else {
            Kind::Dispatch
        };
        Self::new(kind, e)
    }
}

impl StdError for HttpError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

impl fmt::Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("HttpError");

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
            // Kind::Body => f.write_str("request or response body error")?,
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
