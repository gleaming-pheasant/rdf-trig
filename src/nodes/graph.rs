use crate::namespaces::store::NamespaceStore;
use crate::nodes::{IriNode, StagingNode};
use crate::traits::ToStatic;

/// A wrapper around an [`IriNode`] which can optionally be used in a 
/// [`Triple`](crate::triples::Triple).
/// 
/// This can only be constructed by using the [`Into<Graph>`] implementation for 
/// and `IriNode`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Graph<'a>(pub(crate) IriNode<'a>);

impl<'a> Graph<'a> {
    /// Convert this `Graph` into a [`StagingNode`], using the provided 
    /// [`NamepaceStore`] to intern the `Namespace`.
    pub(crate) fn into_staging_node(
        self, namespace_store: &mut NamespaceStore
    ) -> StagingNode<'a> {
        self.0.into_staging(namespace_store)
    }
}

impl<'a> ToStatic for Graph<'a> {
    type StaticType = Graph<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        Graph(self.0.to_static())
    }
}