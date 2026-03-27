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

use crate::WriteNQuads;
use crate::nodes::Node;
use crate::nodes::object::Object;
use crate::traits::ToStatic;
use crate::utils::write_escaped_literal;

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
    /// Create a new `LiteralNode::String` with the provided `str`.
    /// 
    /// Can be used as a placeholder for any literal where the recieving graph 
    /// has no interest in/performs no calculations on the data type.
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

impl<'a> Into<Object<'a>> for &'a LiteralNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Literal(self.clone())
    }
}

impl<'a> Into<Node<'a>> for LiteralNode<'a> {
    fn into(self) -> Node<'a> {
        Node::Literal(self)
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
            LiteralNode::LangString(ls) => LiteralNode::LangString(ls.to_static()),
            LiteralNode::String(s) => {
                LiteralNode::String(Cow::Owned(s.clone().into_owned()))
            }
        }
    }
}

impl<'a> WriteNQuads for LiteralNode<'a> {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            LiteralNode::Boolean(bool) => bool.write_nquads(writer),
            LiteralNode::DateTime(dt) => dt.write_nquads(writer),
            LiteralNode::Decimal(dec) => dec.write_nquads(writer),
            LiteralNode::GYear(gy) => gy.write_nquads(writer),
            LiteralNode::LangString(ls) => ls.write_nquads(writer),
            LiteralNode::String(s) => {
                writer.write_all(b"\"")?;
                write_escaped_literal(writer, &s)?;
                writer.write_all(b"\"")?;

                Ok(())
            }
        }
    }
}