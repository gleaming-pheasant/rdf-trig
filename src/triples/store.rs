use std::ops::Deref;

use super::InternedTriple;

use crate::FastIndexSet;

/// A wrapper around a [`FastIndexSet<InternedTriple>`] which serves to store 
/// unique "triples" and hand out [`InternedTripleId`]s as references to the 
/// [`InternedTriple`]s.
#[derive(Debug)]
pub(crate) struct InternedTripleStore(FastIndexSet<InternedTriple>);

impl InternedTripleStore {
    /// Create a new [`InternedTripleStore`].
    pub(crate) fn new() -> InternedTripleStore {
        InternedTripleStore(FastIndexSet::default())
    }

    /// Add an [`InternedTriple`] to the `InternedTripleStore` returning a 
    /// [`InternedTripleId`].
    /// 
    /// As each element of a triple is a [`NodeId`], which derives [`Clone`] and 
    /// [`Copy`] for the contained [`u32`], this differs from a `NodeStore` and 
    /// `NamespaceStore` in that it does not risk allocation of any types.
    pub(crate) fn intern_triple(
        &mut self, triple: InternedTriple
    ) -> InternedTripleId {
        InternedTripleId::from(self.0.insert_full(triple).0)
    }

    /// Retrieve an iterator over all of the `InternedTriple`s contained in this 
    /// `InternedTripleStore`.
    pub(crate) fn iter(&self) -> indexmap::set::Iter<'_, InternedTriple> {
        self.0.iter()
    }
}

/// A `InternedTripleId` is a wrapper around a `u32` and is only retrievable by 
/// converting the `usize` index from an [`IndexSet`](indexmap::IndexSet) (or a 
/// [`FastIndexSet`] for the purposes of this crate).
/// 
/// This will cause the application to panic if the number of interned nodes 
/// exceeds [`u32::MAX`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedTripleId(u32);

impl InternedTripleId {
    /// Create a new `InternedTripleId` by casting the provided `usize` to a `u32`.
    /// 
    /// Panics if `ix` is greater than [`u32::MAX`].
    pub(crate) fn from(ix: usize) -> InternedTripleId {
        debug_assert!(ix <= u32::MAX as usize);
        InternedTripleId(ix as u32)
    }
}

impl Deref for InternedTripleId {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}