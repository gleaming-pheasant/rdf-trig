mod boolean;
mod datetime;
mod decimal;
mod gyear;
mod string;

pub use boolean::BooleanLiteral;
pub use datetime::DateTimeLiteral;
pub use decimal::DecimalLiteral;
pub use gyear::GYearLiteral;
pub use string::LangStringLiteral;

use std::borrow::Cow;

use crate::nodes::object::Object;
use crate::nodes::store::StagedNode;
use crate::nodes::store::InternedLiteralNode;

/// A wrapper around the possible options that this crate declares for literal 
/// nodes (`GYearLiteral`s, `StringLiteral`s, etc). Each specific type - with 
/// the exception of `String` and `LangString`, which have their own constructor 
/// methods - implements [`Into<LiteralNode>`] for a cleaner API.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum LiteralNode<'a> {
    Boolean(BooleanLiteral),
    DateTime(DateTimeLiteral<'a>),
    Decimal(DecimalLiteral),
    GYear(GYearLiteral),
    LangString(LangStringLiteral<'a>),
    String(Cow<'a, str>)
}

impl<'a> LiteralNode<'a> {
    /// Create a new `LiteralNode::String` with the provided `str`.
    /// 
    /// Can be used as a placeholder for any literal where the recieving graph 
    /// has no interest in the data type.
    pub fn new<C: Into<Cow<'a, str>>>(value: C) -> LiteralNode<'a> {
        LiteralNode::String(value.into())
    }
}

impl<'a> Into<LiteralNode<'a>> for Cow<'a, str> {
    #[inline(always)]
    fn into(self) -> LiteralNode<'a> {
        LiteralNode::String(self)
    }
}

impl<'a> Into<Object<'a>> for Cow<'a, str> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Literal(self.into())
    }
}

impl<'a> Into<Object<'a>> for LiteralNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Literal(self)
    }
}

impl<'a> Into<StagedNode<'a>> for LiteralNode<'a> {
    /// Wrap this `LiteralNode` as a `StagedNode` in preparation for interning.
    #[inline]
    fn into(self) -> StagedNode<'a> {
        StagedNode::Literal(self)
    }
}