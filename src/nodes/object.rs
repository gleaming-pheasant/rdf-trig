use crate::nodes::{BlankNode, NamedNode, Node};
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

impl<'a> Into<Node<'a>> for Object<'a> {
    fn into(self) -> Node<'a> {
        match self {
            Object::Blank(b) => Node::Blank(b),
            Object::Literal(l) => Node::Literal(l),
            Object::Named(n) => Node::Named(n)
        }
    }
}