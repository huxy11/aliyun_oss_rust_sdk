use std::{
    error::Error as StdError,
    fmt::{Display, Error as FmtError, Formatter},
    str::FromStr,
};

#[derive(Clone, Copy, Debug)]
pub enum Schema {
    Http,
    Https,
}

impl Default for Schema {
    fn default() -> Self {
        Schema::Https
    }
}
impl Display for Schema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Schema::Http => "http",
            Schema::Https => "https",
        };
        write!(f, "{}", s)
    }
}
impl FromStr for Schema {
    type Err = ParseSchemaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "http" => Ok(Schema::Http),
            "https" => Ok(Schema::Https),
            _ => Err(ParseSchemaError::new(s)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseSchemaError {
    message: String,
}
impl ParseSchemaError {
    /// Parses Schema given as a string literal
    pub fn new(input: &str) -> Self {
        ParseSchemaError {
            message: format!("Invalid OSS Schema: {}, ", input),
        }
    }
}

impl StdError for ParseSchemaError {}
impl Display for ParseSchemaError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.message)
    }
}
