use std::ops::Deref;

use crate::FastIndexSet;
use crate::namespaces::DEFAULT_GRAPH_NAMESPACE_ID;
use crate::nodes::{StagingIriNode, StagingNode};
use crate::traits::ToInterned;

/// A `NodeId` is a wrapper around a `u32` and is only retrievable by converting 
/// the `usize` index from an [`IndexSet`](indexmap::IndexSet) (or a 
/// [`FastIndexSet`] for the purposes of this crate).
/// 
/// This will cause the application to panic if the number of interned nodes 
/// exceeds [`u32::MAX`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct NodeId(u32);

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

/// The reserved `NodeId` for the default graph's IRI; is set to contain 0 on 
/// initialisation of the [`NodeStore`].
pub(crate) const DEFAULT_GRAPH_NODE_ID: NodeId = NodeId(0);

/// A wrapper around a [`FastIndexSet<StagingNode>`] which serves to store 
/// unique "nodes" and hand out [`NodeId`]s as references to the 
/// interned [`StagingNode`]s.
#[derive(Debug)]
pub(crate) struct NodeStore(FastIndexSet<StagingNode<'static>>);

impl NodeStore {
    /// Create a new `NodeStore`.
    /// 
    /// This function initialises an [`indexmap::IndexSet`] (or [`FastIndexSet`] 
    /// for the purposes of this crate) and inserts a default graph's IRI node 
    /// to guarantee that index 0 is the default graph.
    pub(crate) fn new() -> NodeStore {
        let mut ix_set = FastIndexSet::default();

        ix_set.insert(StagingNode::Iri(
            StagingIriNode::new(DEFAULT_GRAPH_NAMESPACE_ID, "".into()))
        );

        NodeStore(ix_set)
    }

    /// Add a `StagingNode` to this `NodeStore`, returning a `NodeId`.
    /// 
    /// You must retrieve a `StagingNode` from this crate's main `DataStore`, to 
    /// ensure that the `Namespace` for any `IriNode`s has been interened.
    /// 
    /// This crate uses a trait called [`ToInterned`](crate::traits::ToInterned) 
    /// to coerce `'static` lifetimes for any borrowed items during `DataStore` 
    /// building. Therefore, this function calls `to_interned()` - resulting in 
    /// potential allocations - only when it has been established that a Node 
    /// has not already been interned.
    pub(crate) fn intern_node<'a>(&mut self, node: StagingNode<'a>) -> NodeId {
        if let Some(ix) = self.0.get_index_of(&node) {
            return NodeId::from(ix)
        }

        NodeId::from(self.0.insert_full(node.to_interned()).0)
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