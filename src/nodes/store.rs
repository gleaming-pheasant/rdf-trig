use std::ops::Deref;

use crate::FastIndexSet;
use crate::traits::ToStatic;

/// A `NodeId` is a wrapper around a `u32` and is only retrievable by converting 
/// the `usize` index from an [`IndexSet`](indexmap::IndexSet) (or a 
/// [`FastIndexSet`] for the purposes of this crate).
/// 
/// This will cause the application to panic if the number of interned nodes 
/// exceeds [`u32::MAX`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct NodeId(pub(crate) u32);

impl NodeId {
    /// Create a new `NodeId` by casting the provided `usize` to a `u32`.
    /// 
    /// Panics if `ix` is greater than [`u32::MAX`].
    pub(crate) fn from(ix: usize) -> NodeId {
        debug_assert!(ix <= u32::MAX as usize);
        NodeId(ix as u32)
    }
}

impl Deref for NodeId {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A wrapper around a [`FastIndexSet<Node>`] which serves to store 
/// unique "nodes" and hand out [`NodeId`]s as references to the 
/// interned [`Node`]s.
#[derive(Debug)]
pub(crate) struct NodeStore(FastIndexSet<StagingNode<'static>>);

impl NodeStore {
    /// Create a new `NodeStore`.
    pub(crate) fn new() -> NodeStore {
        NodeStore(FastIndexSet::default())
    }

    /// Add a `StagingNode` to this `NodeStore`, returning a `NodeId`.
    /// 
    /// You must retrieve a `StagingNode` from this crate's main `TripleStore`, to 
    /// ensure that the `Namespace` for any `IriNode`s has been interened.
    /// 
    /// This crate uses a trait called [`ToStatic`](crate::traits::ToStatic) 
    /// to coerce `'static` lifetimes for any borrowed items during `TripleStore` 
    /// building. Therefore, this function calls `to_static()` - resulting in 
    /// potential allocations - only when it has been established that a Node 
    /// has not already been interned.
    pub(crate) fn intern_node<'a>(&mut self, node: StagingNode<'a>) -> NodeId {
        if let Some(ix) = self.0.get_index_of(&node) {
            return NodeId::from(ix)
        }

        NodeId::from(self.0.insert_full(node.to_static()).0)
    }

    /// Retrieve a `StagingNode` reference from the provided `NodeId`.
    /// 
    /// Use of [`Option::unwrap`] is considered safe in this function, as the 
    /// crate only allows the generation of `NodeId`s is only through 
    /// [`Self::intern_node`].
    /// 
    /// Any future functionality that allows removal of items from `...Store`s 
    /// must address this.
    pub(crate) fn query_node(&self, node_id: NodeId) -> &StagingNode<'static> {
        self.0.get_index(*node_id as usize).unwrap()
    }
}