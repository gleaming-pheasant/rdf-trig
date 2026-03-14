use crate::namespaces::store::NamespaceStore;
use crate::nodes::{BlankNode, IriNode, StagingNode};

/// A `Subject` forms part of a `Triple`; it defines the node that the 
/// rest of the `Triple` expands upon.
/// 
/// A `Subject` can only be a `BlankNode` or and `IriNode` and so can only be 
/// constructed using the respective [`Into<Subject>`] implementations for these 
/// types.
/// 
/// Without being added to a [`Triple`](crate::triples::Triple) and stored in a 
/// [`TripleStore`](crate::datastore::TripleStore), this struct serves no practical 
/// purpose.
#[derive(Clone, Debug)]
pub enum Subject<'a> {
    Blank(BlankNode<'a>),
    Iri(IriNode<'a>)
}

impl<'a> Subject<'a> {
    /// Convert this `Subject` into a [`StagingNode`], using the provided 
    /// [`NamepaceStore`] to intern the `Namespace` if this is a `Subject::Iri`.
    pub(crate) fn into_staging_node(
        self, namespace_store: &mut NamespaceStore
    ) -> StagingNode<'a> {
        match self {
            Subject::Blank(blank) => blank.into(),
            Subject::Iri(iri) => iri.into_staging(namespace_store)
        }
    }
}

impl<'a> From<&Subject<'a>> for Subject<'a> {
    fn from(s: &Subject<'a>) -> Self {
        s.clone()
    }
}