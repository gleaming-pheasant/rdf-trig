use crate::namespaces::store::NamespaceStore;
use crate::nodes::{BlankNode, IriNode, StagingNode};
use crate::nodes::literals::LiteralNode;

/// An `Object` is the final part of any `Triple`, effectively providing the 
/// value of a `Predicate` for a `Subject`.
/// 
/// An `Object` can be any of a `BlankNode`, `IriNode` or a literal node, and 
/// can be constructed using the [`Into<Object>`] implementations of any of 
/// those types.
/// 
/// Without being added to a [`Triple`](crate::triples::Triple) and stored in a 
/// [`TripleStore`](crate::datastore::TripleStore), this struct serves no practical 
/// purpose.
#[derive(Clone, Debug)]
pub enum Object<'a> {
    Blank(BlankNode<'a>),
    Iri(IriNode<'a>),
    Literal(LiteralNode<'a>)
}

impl<'a> Object<'a> {
    /// Convert this `Object` into a [`StagingNode`], using the provided 
    /// [`NamepaceStore`] to intern the `Namespace` if this is a `Object::Iri`.
    pub(crate) fn into_staging_node(
        self, namespace_store: &mut NamespaceStore
    ) -> StagingNode<'a> {
        match self {
            Object::Blank(blank) => blank.into(),
            Object::Iri(iri) => iri.into_staging(namespace_store),
            Object::Literal(literal) => literal.into()
        }
    }
}

impl<'a> From<&Object<'a>> for Object<'a> {
    fn from(o: &Object<'a>) -> Self {
        o.clone()
    }
}