mod boolean;
mod datetime;
mod decimal;
mod gyear;
mod string;

pub use boolean::BooleanLiteral;
pub use datetime::DateTimeLiteral;
pub use decimal::DecimalLiteral;
pub use gyear::GYearLiteral;
pub use string::StringLiteral;

use std::borrow::Cow;
use std::io::{self, Write};

use crate::traits::{ToStatic, WriteNQuads, WriteTriG};

/// A wrapper around the possible options that this crate declares for literal 
/// nodes (`GYearLiteral`s, `StringLiteral`s, etc). Each specific type - with 
/// the exception of `String` and `LangString`, which have their own constructor 
/// methods - implements [`Into<LiteralNode>`] for a cleaner API.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LiteralNode<'a> {
    Boolean(BooleanLiteral),
    DateTime(DateTimeLiteral<'a>),
    Decimal(DecimalLiteral),
    GYear(GYearLiteral),
    String(StringLiteral<'a>)
}

impl<'a> LiteralNode<'a> {
    /// Create a new `LiteralNode::String` with the provided 
    /// `Into<Cow<'a, str>>`.
    /// 
    /// Does not define the language for the contained `StringLiteral`.
    /// 
    /// Can be used as a placeholder for any literal where the recieving graph 
    /// has no interest in/performs no calculations on the data type.
    pub fn new<C: Into<Cow<'a, str>>>(value: C) -> LiteralNode<'a> {
        LiteralNode::String(
            StringLiteral::new(value.into(), None::<Cow<'a, str>>)
                .unwrap() // Safe because new() only fails where language is Some
        )
    }
}

impl<'a> From<BooleanLiteral> for LiteralNode<'a> {
    #[inline(always)]
    fn from(value: BooleanLiteral) -> LiteralNode<'a> {
        LiteralNode::Boolean(value)
    }
}

impl<'a> From<DateTimeLiteral<'a>> for LiteralNode<'a> {
    #[inline(always)]
    fn from(value: DateTimeLiteral<'a>) -> LiteralNode<'a> {
        LiteralNode::DateTime(value)
    }
}

impl<'a> From<DecimalLiteral> for LiteralNode<'a> {
    #[inline(always)]
    fn from(value: DecimalLiteral) -> LiteralNode<'a> {
        LiteralNode::Decimal(value)
    }
}

impl<'a> From<GYearLiteral> for LiteralNode<'a> {
    #[inline(always)]
    fn from(value: GYearLiteral) -> LiteralNode<'a> {
        LiteralNode::GYear(value)
    }
}

impl<'a> From<StringLiteral<'a>> for LiteralNode<'a> {
    #[inline(always)]
    fn from(value: StringLiteral<'a>) -> LiteralNode<'a> {
        LiteralNode::String(value)
    }
}

impl<'a> From<Cow<'a, str>> for LiteralNode<'a> {
    #[inline(always)]
    fn from(value: Cow<'a, str>) -> LiteralNode<'a> {
        LiteralNode::String(
            StringLiteral::new(value, None::<Cow<'a,str>>)
                .unwrap() // Safe - see `LiteralNode::new()`
        )
    }
}

impl<'a> ToStatic for LiteralNode<'a> {
    type StaticType = LiteralNode<'static>;

    fn to_static(&self) -> Self::StaticType {
        match self {
            LiteralNode::Boolean(bool) => LiteralNode::Boolean(*bool),
            LiteralNode::DateTime(dtl) => LiteralNode::DateTime(dtl.to_static()),
            LiteralNode::Decimal(dec) => LiteralNode::Decimal(*dec),
            LiteralNode::GYear(int) => LiteralNode::GYear(*int),
            LiteralNode::String(ls) => LiteralNode::String(ls.to_static())
        }
    }
}

impl<'a> WriteNQuads for LiteralNode<'a> {
    #[inline]
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            LiteralNode::Boolean(bool) => bool.write_nquads(writer),
            LiteralNode::DateTime(dt) => dt.write_nquads(writer),
            LiteralNode::Decimal(dec) => dec.write_nquads(writer),
            LiteralNode::GYear(gy) => gy.write_nquads(writer),
            LiteralNode::String(ls) => ls.write_nquads(writer)
        }
    }
}

impl<'a> WriteTriG for LiteralNode<'a> {
    #[inline]
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            LiteralNode::Boolean(bool) => bool.write_trig(writer),
            LiteralNode::DateTime(dt) => dt.write_trig(writer),
            LiteralNode::Decimal(dec) => dec.write_trig(writer),
            LiteralNode::GYear(gy) => gy.write_trig(writer),
            LiteralNode::String(ls) => ls.write_trig(writer)
        }
    }
}