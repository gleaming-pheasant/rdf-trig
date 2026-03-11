use core::f32;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};

use crate::WriteTriG;
use crate::errors::RdfTrigError;
use crate::nodes::object::Object;
use crate::nodes::literals::LiteralNode;

/// A wrapper around an [`f32`], which can be constructed either with a 
/// native `f32`, or with a string which can be parsed as one.
/// 
/// Values in this struct are stored as `f32`s and output with a custom `f32` 
/// formatting in [`WriteTriG`], which appends a period (".") if the value is a 
/// whole number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecimalLiteral(f32);

impl Eq for DecimalLiteral {}

impl Hash for DecimalLiteral {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let canonical = self.0 + 0.0;

        let bits = if canonical.is_nan() {
            0x7fc00000u32
        } else {
            canonical.to_bits()
        };

        bits.hash(state);
    }
}

impl DecimalLiteral {
    /// Create a new `DecimalLiteral` from a `str`-like type. The `str` must be 
    /// parsable as an `f32` or this function will return an error.
    pub fn from_str<'a, C: Into<Cow<'a, str>>>(value: C)
    -> Result<DecimalLiteral, RdfTrigError> {
        let value = value.into();
        
        if let Ok(decimal) = value.parse::<f32>() {
            Ok(DecimalLiteral(decimal))
        } else {
            Err(RdfTrigError::InvalidDecimal(value.to_string()))
        }
    }
}

impl From<f32> for DecimalLiteral {
    #[inline]
    fn from(value: f32) -> Self {
        DecimalLiteral(value)
    }
}

impl<'a> Into<LiteralNode<'a>> for DecimalLiteral {
    #[inline(always)]
    fn into(self) -> LiteralNode<'a> {
        LiteralNode::Decimal(self)
    }
}

impl<'a> Into<Object<'a>> for DecimalLiteral {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Literal(self.into())
    }
}

impl WriteTriG for DecimalLiteral {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.0.to_string().as_bytes())?;
        if self.0.fract() == 0.0 {
            writer.write_all(b".")?;
        }
        Ok(())
    }    
}