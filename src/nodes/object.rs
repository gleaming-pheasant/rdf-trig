use std::borrow::Cow;

use crate::nodes::{
    BlankNode,
    BooleanLiteral,
    DateTimeLiteral,
    DecimalLiteral,
    GYearLiteral,
    NamedNode,
    StringLiteral
};
use crate::nodes::literals::LiteralNode;

/// An `Object` is the final part of any `Triple`, effectively providing the 
/// value of a `Predicate` for a `Subject`.
/// 
/// An `Object` can be any of a `BlankNode`, `NamedNode` or a literal node, and 
/// can be constructed using the [`Into<Object>`] implementations of any of 
/// those types.
/// 
/// Without being added to a [`Triple`](crate::triples::Triple) and stored in a 
/// [`TripleStore`](crate::datastore::TripleStore), this struct serves no practical 
/// purpose.
#[derive(Clone, Debug)]
pub enum Object<'a> {
    Blank(BlankNode<'a>),
    Literal(LiteralNode<'a>),
    Named(NamedNode<'a>)
}

impl<'a> Object<'a> {
    pub(crate) fn new_const_named(iri: &'static str) -> Object<'static> {
        Object::Named(NamedNode::new_const(iri))
    }
}

impl<'a> From<&Object<'a>> for Object<'a> {
    fn from(o: &Object<'a>) -> Self {
        o.clone()
    }
}

impl<'a> From<Cow<'a, str>> for Object<'a> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Object<'a> {
        Object::Literal(value.into())
    }
}

impl<'a> From<BooleanLiteral> for Object<'a> {
    #[inline(always)]
    fn from(value: BooleanLiteral) -> Object<'a> {
        Object::Literal(value.into())
    }
}

impl<'a> From<DateTimeLiteral<'a>> for Object<'a> {
    #[inline(always)]
    fn from(value: DateTimeLiteral<'a>) -> Object<'a> {
        Object::Literal(value.into())
    }
}

impl<'a> From<DecimalLiteral> for Object<'a> {
    #[inline(always)]
    fn from(value: DecimalLiteral) -> Object<'a> {
        Object::Literal(value.into())
    }
}

impl<'a> From<GYearLiteral> for Object<'a> {
    #[inline(always)]
    fn from(value: GYearLiteral) -> Object<'a> {
        Object::Literal(value.into())
    }
}

impl<'a> From<StringLiteral<'a>> for Object<'a> {
    #[inline(always)]
    fn from(value: StringLiteral<'a>) -> Object<'a> {
        Object::Literal(value.into())
    }
}

impl<'a> From<LiteralNode<'a>> for Object<'a> {
    #[inline(always)]
    fn from(value: LiteralNode<'a>) -> Object<'a> {
        Object::Literal(value.into())
    }
}

impl<'a> From<NamedNode<'a>> for Object<'a> {
    #[inline]
    fn from(value: NamedNode<'a>) -> Object<'a> {
        Object::Named(value)
    }
}

impl<'a> From<&NamedNode<'a>> for Object<'a> {
    #[inline]
    fn from(value: &NamedNode<'a>) -> Object<'a> {
        Object::Named(value.clone())
    }
}

impl<'a> From<BlankNode<'a>> for Object<'a> {
    #[inline]
    fn from(value: BlankNode<'a>) -> Object<'a> {
        Object::Blank(value)
    }
}

impl<'a> From<&BlankNode<'a>> for Object<'a> {
    #[inline]
    fn from(value: &BlankNode<'a>) -> Object<'a> {
        Object::Blank(value.clone())
    }
}