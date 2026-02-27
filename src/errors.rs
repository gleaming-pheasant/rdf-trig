//! Contains errors that can be returned by verifying types.
use std::borrow::Cow;

#[derive(Debug)]
pub enum RdfLiteError {
    InvalidBoolean(Cow<'static, str>),
    InvalidDateTime(Cow<'static, str>),
    InvalidDecimal(Cow<'static, str>),
    InvalidGYear(Cow<'static, str>)
}

impl std::fmt::Display for RdfLiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfLiteError::InvalidBoolean(bool) => {
                write!(f, "{} is an invalid boolean", bool)
            },
            RdfLiteError::InvalidDateTime(dt) => {
                write!(f, "{} is an invalid dateTime", dt)
            },
            RdfLiteError::InvalidDecimal(dec) => {
                write!(f, "{} is an invalid decimal", dec)
            },
            RdfLiteError::InvalidGYear(gy) => {
                write!(f, "{} is an invalid gYear", gy)
            }
        }
    }
}

impl std::error::Error for RdfLiteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Update if any external errors are required.
        None
    }
}