use std::borrow::Borrow;

use indexmap::Equivalent;

use crate::nodes::BlankNode;

/// A `BlankNode` which has been interned by eliciting a String from the 
/// `Cow<'a, str>`.
/// 
/// The original `BlankNode` containing a reference can still be used to check 
/// for the presence of a matching owned `InternedBlankNode`.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedBlankNode(pub BlankNode<'static>);

impl PartialEq<BlankNode<'_>> for InternedBlankNode {
    fn eq(&self, other: &BlankNode<'_>) -> bool {
        self.0 == *other
    }
}

impl Equivalent<BlankNode<'_>> for InternedBlankNode {
    #[inline]
    fn equivalent(&self, key: &BlankNode<'_>) -> bool {
        self == key
    }
}

impl<'a> Borrow<BlankNode<'a>> for InternedBlankNode {
    #[inline]
    fn borrow(&self) -> &BlankNode<'a> {
        &self.0
    }
}