use crate::nodes::{BlankNode, NamedNode, Node};

/// A `Subject` forms part of a `Triple`; it defines the node that the 
/// rest of the `Triple` expands upon.
/// 
/// A `Subject` can only be a `BlankNode` or and `NamedNode` and so can only be 
/// constructed using the respective [`Into<Subject>`] implementations for these 
/// types.
/// 
/// Without being added to a [`Triple`](crate::triples::Triple) and stored in a 
/// [`TripleStore`](crate::triplestore::TripleStore), this struct serves no 
/// practical purpose.
#[derive(Clone, Debug)]
pub enum Subject<'a> {
    Blank(BlankNode<'a>),
    Named(NamedNode<'a>)
}

impl<'a> Subject<'a> {
    pub(crate) fn new_const_named(iri: &'static str) -> Subject<'static> {
        Subject::Named(NamedNode::new_const(iri))
    }
}

impl<'a> From<&Subject<'a>> for Subject<'a> {
    fn from(s: &Subject<'a>) -> Self {
        s.clone()
    }
}

impl<'a> Into<Node<'a>> for Subject<'a> {
    fn into(self) -> Node<'a> {
        match self {
            Subject::Blank(b) => Node::Blank(b),
            Subject::Named(n) => Node::Named(n)
        }
    }
}