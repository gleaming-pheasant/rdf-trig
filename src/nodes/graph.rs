use crate::nodes::NamedNode;
use crate::traits::ToStatic;

/// A wrapper around an [`NamedNode`] which can optionally be used in a 
/// [`Triple`](crate::triples::Triple).
/// 
/// This can only be constructed by using the [`Into<Graph>`] implementation for 
/// and [`NamedNode`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph<'a>(pub(crate) NamedNode<'a>);

impl<'a> ToStatic for Graph<'a> {
    type StaticType = Graph<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        Graph(self.0.to_static())
    }
}