use std::borrow::Cow;
use std::io::{self, Write};

use crate::errors::RdfTrigError;
use crate::traits::{WriteNQuads, WriteTriG};

// Unfortunately these can't be concat!-ed to a single static string from consts.
const XSD_BOOLEAN_IRI: &'static str = "<http://www.w3.org/2001/XMLSchema#boolean>";

/// A wrapper around a [`bool`], which can be constructed either with a 
/// native `bool`, or with a string equal to "1"/"0" or "true"/"false".
/// 
/// Values in this struct are stored as `bool`s and output as the `bool` 
/// standard [`ToString`] values of "true" or "false" - regardless of the input 
/// value - in order to reduce memory usage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BooleanLiteral(pub(crate) bool);

impl BooleanLiteral {
    /// Create a new `BooleanLiteral` from a `str` type value.
    /// 
    /// The given value must be either "true", "false", "1" or "0", or it will 
    /// return an error.
    /// 
    // This is a custom function (not TryFrom), to accept Into<Cow...> values. 
    pub fn try_from_str<'a, C: Into<Cow<'a, str>>>(value: C)
    -> Result<BooleanLiteral, RdfTrigError> {
        let value = value.into();
        match &*value {
            "1" | "true" => Ok(BooleanLiteral(true)),
            "0" | "false" => Ok(BooleanLiteral(false)),
            _ => Err(RdfTrigError::InvalidBoolean(value.to_string()))
        }
    }
}

impl From<bool> for BooleanLiteral {
    #[inline]
    fn from(value: bool) -> Self {
        BooleanLiteral(value)
    }
}

impl TryFrom<u8> for BooleanLiteral {
    type Error = RdfTrigError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BooleanLiteral(false)),
            1 => Ok(BooleanLiteral(true)),
            _ => Err(RdfTrigError::InvalidBoolean(value.to_string()))
        }
    }
}

impl WriteNQuads for BooleanLiteral {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"\"")?;
        writer.write_all(self.0.to_string().as_bytes())?;
        writer.write_all(b"\"^^")?;
        writer.write_all(XSD_BOOLEAN_IRI.as_bytes())?;
        Ok(())
    }
}

impl WriteTriG for BooleanLiteral {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.0.to_string().as_bytes())?;
        Ok(())
    }
}