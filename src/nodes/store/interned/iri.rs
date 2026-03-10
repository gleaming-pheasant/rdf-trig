use std::borrow::Borrow;

use indexmap::Equivalent;

use crate::namespaces::NamespaceId;
use crate::nodes::store::staged::StagedIriNode;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedIriNode(pub StagedIriNode<'static>);

impl PartialEq<StagedIriNode<'_>> for InternedIriNode {
    fn eq(&self, other: &StagedIriNode<'_>) -> bool {
        self.namespace_id == other.namespace_id() && 
        *self.endpoint == *other.endpoint()
    }
}

impl Equivalent<StagedIriNode<'_>> for InternedIriNode {
    #[inline]
    fn equivalent(&self, key: &StagedIriNode<'_>) -> bool {
        self == key
    }
}

impl<'a> Borrow<StagedIriNode<'a>> for InternedIriNode {
    fn borrow(&self) -> &StagedIriNode<'a> {
        &self.0
    }
}