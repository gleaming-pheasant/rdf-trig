use std::borrow::Cow;

use crate::namespaces::NamespaceId;
use crate::nodes::store::interned::{
    InternedBlankNode,
    InternedIriNode,
    InternedNode
};
use crate::nodes::{BlankNode, LiteralNode};

/// A staging ground for Nodes prior to interning in a `NodeStore`. Provides a 
/// place to store nodes prior to interning; a simple wrapper around the same 
/// node types as [`Node`](crate::nodes::Node), but with Iri replaced with a 
/// [`StagedIriNode`], containing a previously interned `Namespace` for the IRI.
pub(crate) enum StagedNode<'a> {
    Blank(BlankNode<'a>),
    Iri(StagedIriNode<'a>),
    Literal(LiteralNode<'a>)
}

impl<'a> ToOwned for StagedNode<'a> {
    type Owned = InternedNode;

    fn to_owned(&self) -> Self::Owned {
        match self {
            StagedNode::Blank(blank) => InternedNode::Blank(blank.to_owned()),
            StagedNode::Iri(iri) => InternedNode::Iri(iri.to_owned()),
            StagedNode::Literal(literal) => {

            }
        }
    }
}

/// An [`IriNode`] that stores an already interned 
/// [`Namespace`](crate::namespaces::Namespace)'s `NamespaceId`.
/// 
/// This type still retains its lifetime, as it can still reference a temporary 
/// value prior to the interning of the node itself.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StagedIriNode<'a> {
    namespace_id: NamespaceId,
    endpoint: Cow<'a, str>
}

impl<'a> StagedIriNode<'a> {
    /// Create a new `StagedIriNode` from a retrieved [`NamespaceId`] and an 
    /// `endpoint`.
    pub(crate) fn new(
        namespace_id: NamespaceId, endpoint: Cow<'a, str>
    ) -> StagedIriNode<'a> {
        StagedIriNode { namespace_id, endpoint }
    }

    /// Get the `namespace_id` for this `StagedIriNode`.
    pub(crate) fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    /// Get a reference to the `endpoint` for this `StagedIriNode`.
    pub(crate) fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub(crate) fn to_interned(&self) -> InternedIriNode {
        InternedIriNode(
            StagedIriNode {
                namespace_id: self.namespace_id.clone(),
                endpoint: Cow::Owned(self.endpoint.to_string())
            }
        )
    }
}

impl<'a> Into<StagedNode<'a>> for StagedIriNode<'a> {
    /// Wrap this `StagedIriNode` as a `StagedNode` in preparation for interning.
    #[inline]
    fn into(self) -> StagedNode<'a> {
        StagedNode::Iri(self)
    }
}


impl ToOwned for StagedIriNode<'_> {
    type Owned = InternedIriNode;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_interned()
    }
}