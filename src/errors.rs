//! Contains errors that can be returned by verifying types.
use std::borrow::Cow;

#[derive(Debug)]
pub enum RdfTrigError<'a> {
    InvalidBoolean(Cow<'a, str>),
    InvalidDateTime(Cow<'a, str>),
    InvalidDecimal(Cow<'a, str>),
    InvalidGYear(Cow<'a, str>),
    InvalidIri(Cow<'a, str>),
    InvalidLanguage(Cow<'a, str>)
}

impl<'a> std::fmt::Display for RdfTrigError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfTrigError::InvalidBoolean(bool) => {
                write!(f, "{} is an invalid boolean", bool)
            },
            RdfTrigError::InvalidDateTime(dt) => {
                write!(f, "{} is an invalid dateTime", dt)
            },
            RdfTrigError::InvalidDecimal(dec) => {
                write!(f, "{} is an invalid decimal", dec)
            },
            RdfTrigError::InvalidGYear(gy) => {
                write!(f, "{} is an invalid gYear", gy)
            },
            RdfTrigError::InvalidIri(lang) => {
                write!(f, "{} is an IRI component", lang)
            },
            RdfTrigError::InvalidLanguage(lang) => {
                write!(f, "{} is an invalid language code", lang)
            }
        }
    }
}

impl<'a> std::error::Error for RdfTrigError<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Update if any external errors are required.
        None
    }
}