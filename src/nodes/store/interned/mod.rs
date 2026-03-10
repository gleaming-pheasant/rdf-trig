mod blank;
mod iri;
mod literals;

use std::borrow::Borrow;

pub(crate) use blank::InternedBlankNode;
pub(crate) use iri::InternedIriNode;
pub(crate) use literals::InternedLiteralNode;

use indexmap::Equivalent;

use crate::nodes::store::StagedNode;

/// An enum over all possible interned node types. Each option represents a node 
/// with all potentially borrowed values (typically `Cow<'a, str>`) converted to 
/// owned.
/// 
/// Each contained type also implements [`Equivalent`](indexmap::Equivalent) to 
/// their un-interned counterparts (e.g. `impl Equivalent<IriNode<'_>> for 
/// InternedIriNode`) in order to be able to query values in a `NodeStore`, 
/// without requiring allocation of parts (i.e. converting `Cow<'a, str>` to 
/// `String`) for the sole purposes of hashing.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum InternedNode {
    Blank(InternedBlankNode),
    Iri(InternedIriNode), // Fetch the NamespaceId to Staged first, then intern.
    Literal(InternedLiteralNode)
}

impl<'a> Equivalent<StagedNode<'a>> for InternedNode {
    fn equivalent(&self, key: &StagedNode<'a>) -> bool {
        self == key
    }
}

impl<'a> PartialEq<StagedNode<'a>> for InternedNode {
    #[inline]
    fn eq(&self, other: &StagedNode<'a>) -> bool {
        match (self, other) {
            (Self::Blank(s), StagedNode::Blank(o)) => s == o,
            (Self::Iri(s), StagedNode::Iri(o)) => s == o,
            (Self::Literal(s), StagedNode::Literal(o)) => s == o,
            _ => false
        }
    }
}

impl<'a> Borrow<StagedNode<'a>> for InternedNode {
    fn borrow(&self) -> &StagedNode<'a> {
        match self {
            InternedNode::Blank(blank) => ,
            Int
        }
    }
}