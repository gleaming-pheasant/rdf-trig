use indexmap::Equivalent;

use crate::nodes::raw::literals::{
    BooleanLiteral,
    DateTimeLiteral,
    DecimalLiteral,
    GYearLiteral,
    LangStringLiteral,
    LiteralNode
};

/// One of the `LiteralNode` types which has been interned by eliciting owned 
/// String values from `Cow<'a, str>`s.
/// 
/// The original `LiteralNode` containing a reference can still be used to check 
/// for the presence of a matching owned `InternedBlankNode`.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum InternedLiteralNode {
    Boolean(BooleanLiteral),
    DateTime(InternedDateTimeLiteral),
    Decimal(DecimalLiteral),
    GYear(GYearLiteral),
    LangString(InternedLangStringLiteral),
    String(String)
}

impl<'a> PartialEq<LiteralNode<'a>> for InternedLiteralNode {
    #[inline]
    fn eq(&self, other: &LiteralNode<'a>) -> bool {
        match (self, other) {
            (Self::Boolean(s), LiteralNode::Boolean(o)) => s == o,
            (Self::Decimal(s), LiteralNode::Decimal(o)) => s == o,
            (Self::GYear(s), LiteralNode::GYear(o)) => s == o,
            (Self::String(s), LiteralNode::String(o)) => s == o,
            // Leverage existing Equivalent impls for complex types
            (Self::DateTime(s), LiteralNode::DateTime(o)) => s == o,
            (Self::LangString(s), LiteralNode::LangString(o)) => s == o,
            _ => false
        }
    }
}

impl Equivalent<LiteralNode<'_>> for InternedLiteralNode {
    #[inline]
    fn equivalent(&self, key: &LiteralNode<'_>) -> bool {
        self == key
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedDateTimeLiteral(String);

impl Equivalent<DateTimeLiteral<'_>> for InternedDateTimeLiteral {
    fn equivalent(&self, key: &DateTimeLiteral<'_>) -> bool {
        *self.0 == *key.0
    }
}

impl PartialEq<DateTimeLiteral<'_>> for InternedDateTimeLiteral {
    fn eq(&self, other: &DateTimeLiteral<'_>) -> bool {
        *self.0 == *other.0
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct InternedLangStringLiteral {
    value: String,
    language: String
}

impl Equivalent<LangStringLiteral<'_>> for InternedLangStringLiteral {
    fn equivalent(&self, key: &LangStringLiteral<'_>) -> bool {
        *self.value == *key.value() && *self.language == *key.language()
    }
}

impl PartialEq<LangStringLiteral<'_>> for InternedLangStringLiteral {
    fn eq(&self, other: &LangStringLiteral<'_>) -> bool {
        *self.value == *other.value() && *self.language == *other.language()
    }
}