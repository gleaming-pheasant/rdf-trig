use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};

use crate::errors::RdfTrigError;
use crate::traits::{WriteNQuads, WriteTriG};

const XSD_DECIMAL_IRI: &'static str = "<http://www.w3.org/2001/XMLSchema#decimal>";

/// A wrapper around an [`f64`], which can be constructed either with a 
/// native `f64`, or with a string which can be parsed as one.
/// 
/// Techincally this is invalid, a Rust `f64` should translate to an XSD 
/// "double", but this is not commonly used. It is hoped receiving parsers can 
/// handle/truncate long `f64` values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecimalLiteral(f64);

impl Eq for DecimalLiteral {}

impl Hash for DecimalLiteral {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let canonical = self.0 + 0.0;

        let bits = if canonical.is_nan() {
            0x7fc00000f64
        } else {
            canonical.to_bits()
        };

        bits.hash(state);
    }
}

impl DecimalLiteral {
    /// Create a new `DecimalLiteral` from a `str`-like type. The `str` must be 
    /// parsable as an `f32` or this function will return an error.
    pub fn try_from_str<'a, C: Into<Cow<'a, str>>>(value: C)
    -> Result<DecimalLiteral, RdfTrigError> {
        let value = value.into();
        
        if let Ok(decimal) = value.parse::<f64>() {
            Ok(DecimalLiteral(decimal))
        } else {
            Err(RdfTrigError::InvalidDecimal(value.to_string()))
        }
    }
}

impl From<f64> for DecimalLiteral {
    #[inline]
    fn from(value: f64) -> Self {
        DecimalLiteral(value)
    }
}

impl WriteNQuads for DecimalLiteral {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"\"")?;
        writer.write_all(self.0.to_string().as_bytes())?;
        if self.0.fract() == 0.0 {
            writer.write_all(b".")?;
        }
        writer.write_all(b"\"^^")?;
        writer.write_all(XSD_DECIMAL_IRI.as_bytes())?;
        Ok(())
    }
}

impl WriteTriG for DecimalLiteral {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.0.to_string().as_bytes())?;
        // Trailling period is to ensure this is captured as a decimal by TriG 
        // parsers where the fractional part is 0. This is because decimal types 
        // are inferred in TriG by a trailling decimal.
        if self.0.fract() == 0.0 {
            writer.write_all(b".")?;
        }

        Ok(())
    }
}