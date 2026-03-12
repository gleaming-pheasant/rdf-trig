use crate::nodes::IriNode;
use crate::traits::ToStatic;

/// A wrapper around an [`IriNode`] which can optionally be used in a 
/// [`Triple`](crate::triples::Triple).
/// 
/// This can only be constructed by using the [`Into<Graph>`] implementation for 
/// and `IriNode`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph<'a>(pub(crate) IriNode<'a>);

impl<'a> ToStatic for Graph<'a> {
    type StaticType = Graph<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        Graph(self.0.to_static())
    }
}