use std::borrow::Cow;
use std::io::{self, Write};

use crate::WriteNQuads;
use crate::errors::RdfTrigError;
use crate::nodes::object::Object;
use crate::nodes::literals::LiteralNode;

const XSD_GYEAR: &[u8; 40] = b"<http://www.w3.org/2001/XMLSchema#gYear>";

/// A wrapper around an [`i32`], which can be constructed either with a 
/// native `i32`, or with a string which can be parsed as one.
/// 
/// Values in this struct are stored as `i32`s and output with the standard 
/// [`ToString`] implementation for `i32`.
/// 
/// This is not a valid XML Schema gYear. It does not pad years with fewer than 
/// four digits with zeroes and does not allow timezone declarations. This is to 
/// assist with speed an practicality of processing.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GYearLiteral(i32);

impl GYearLiteral {
    /// Create a new `GYearLiteral` from a `str`-like type. The `str` must be 
    /// parsable as an `i32` or this function will return an error.
    pub fn try_from_str<'a, C: Into<Cow<'a, str>>>(value: C)
    -> Result<GYearLiteral, RdfTrigError> {
        let value = value.into();
        if let Ok(year) = value.parse::<i32>() {
            Ok(GYearLiteral(year))
        } else {
            Err(RdfTrigError::InvalidGYear(value.to_string()))
        }
    }
}

impl From<i32> for GYearLiteral {
    #[inline]
    fn from(value: i32) -> Self {
        GYearLiteral(value)
    }
}

impl<'a> Into<LiteralNode<'a>> for GYearLiteral {
    #[inline(always)]
    fn into(self) -> LiteralNode<'a> {
        LiteralNode::GYear(self)
    }
}

impl<'a> Into<Object<'a>> for GYearLiteral {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Literal(self.into())
    }
}

impl WriteNQuads for GYearLiteral {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"\"")?;
        writer.write_all(self.0.to_string().as_bytes())?;
        writer.write_all(b"\"^^")?;
        writer.write_all(XSD_GYEAR)?;

        Ok(())
    }
}
