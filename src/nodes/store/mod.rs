mod interned;
mod staged;
// mod view;

pub(crate) use interned::{
    InternedBlankNode,
    InternedIriNode,
    InternedLiteralNode,
    InternedNode
};
pub(crate) use staged::StagedNode;

use std::ops::Deref;

use crate::FastIndexSet;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct NodeId(u32);

impl NodeId {
    pub(crate) fn from(ix: usize) -> NodeId {
        debug_assert!(ix <= u32::MAX as usize);
        NodeId(ix as u32)
    }
}

impl Deref for NodeId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A wrapper around a [`FastIndexSet<InternedNode>`] which serves to store 
/// unique "nodes" and hand out [`NodeId`]s as references to the 
/// [`InternedNode`]s.
#[derive(Debug)]
pub struct NodeStore(FastIndexSet<InternedNode>);

impl NodeStore {
    /// Create a new `NodeStore`.
    pub(crate) fn new() -> NodeStore {
        NodeStore(FastIndexSet::default())
    }

    /// Add a `StagedNode` to this `NodeStore`, returning a `NodeId`.
    /// 
    /// You must retrieve a `StagedNode` from the `DataStore` main entrypoint to 
    /// this crate, in order to ensure that the `Namespace` for any `IriNode`s 
    /// has been interened.
    /// 
    /// This function uses the implementations of [`Equivalent`] and - by 
    /// association - [`PartialEq`] of `StagedNode` to `InternedNode` to allow 
    /// checking for previously interned data without allocation.
    pub(crate) fn intern_node<'a>(&mut self, node: StagedNode<'a>) -> NodeId {
        let endpoint = endpoint.into();
        let key = InternedGraphSearchKey {namespace_id, endpoint: &endpoint};

        if let Some(ix) = self.store.get_index_of(&key) {
            return GraphId::from(ix)
        }

        let owned = InternedGraph {
            namespace_id, endpoint: Cow::Owned(endpoint.into_owned())
        };

        self.intern_graph(owned)

        NodeId::from(self.store.insert_full(node).0)
    }

    /// Retrieve an `InternedNode` from the provided `NodeId`.
    pub(crate) fn query_node(&self, node_id: NodeId) -> &InternedNode {
        self.store.get_index(*node_id as usize).unwrap()
    }
}