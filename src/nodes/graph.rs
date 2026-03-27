use crate::nodes::{NamedNode, Node};
use crate::traits::ToStatic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph<'a>(pub(crate) NamedNode<'a>);

impl<'a> Graph<'a> {
    /// Create a new `Graph` from 'static values. Only accessible within 
    /// this crate to bypass IRI validation.
    pub(crate) const fn new_const(iri: &'static str) -> Graph<'a> {
        Graph(NamedNode::new_const(iri))
    }
}

impl<'a> From<&Graph<'a>> for Graph<'a> {
    fn from(p: &Graph<'a>) -> Self {
        p.clone()
    }
}

impl<'a> ToStatic for Graph<'a> {
    type StaticType = Graph<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        Graph(self.0.to_static())
    }
}

impl<'a> Into<Node<'a>> for Graph<'a> {
    fn into(self) -> Node<'a> {
        Node::Named(self.0)
    }
}