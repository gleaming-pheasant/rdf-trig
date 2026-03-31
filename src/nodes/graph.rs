use crate::nodes::{BlankNode, NamedNode};
use crate::traits::ToStatic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Graph<'a> {
    Blank(BlankNode<'a>),
    Named(NamedNode<'a>)
}

impl<'a> Graph<'a> {
    /// Create a new `Graph` from 'static values. Only accessible within 
    /// this crate to bypass IRI validation.
    pub(crate) const fn new_const_named(iri: &'static str) -> Graph<'a> {
        Graph::Named(NamedNode::new_const(iri))
    }
}

impl<'a> From<&Graph<'a>> for Graph<'a> {
    fn from(p: &Graph<'a>) -> Self {
        p.clone()
    }
}

impl<'a> From<BlankNode<'a>> for Graph<'a> {
    #[inline]
    fn from(value: BlankNode<'a>) -> Graph<'a> {
        Graph::Blank(value)
    }
}

impl<'a> From<&BlankNode<'a>> for Graph<'a> {
    #[inline]
    fn from(value: &BlankNode<'a>) -> Graph<'a> {
        Graph::Blank(value.clone())
    }
}

impl<'a> From<NamedNode<'a>> for Graph<'a> {
    #[inline]
    fn from(value: NamedNode<'a>) -> Graph<'a> {
        Graph::Named(value)
    }
}

impl<'a> From<&NamedNode<'a>> for Graph<'a> {
    #[inline]
    fn from(value: &NamedNode<'a>) -> Graph<'a> {
        Graph::Named(value.clone())
    }
}

impl<'a> ToStatic for Graph<'a> {
    type StaticType = Graph<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        match self {
            Graph::Blank(b) => Graph::Blank(b.to_static()),
            Graph::Named(n) => Graph::Named(n.to_static())
        }
    }
}