//! Contains index types which are just different ways of organising interned 
//! object IDs.

use std::ops::{Deref, DerefMut};

use ahash::{HashMap, HashMapExt};

use crate::{nodes::NodeId, triples::InternedTripleId};

/// Associates a `Graph` with all of its known `InternedTriple`s.
/// 
/// This keeps a `HashMap` of the `NodeId` for a graph with a collection of
/// `InternedTripleId`s of the `InternedTriple`s for which said `Graph` is a 
/// part.
#[derive(Debug)]
pub(crate) struct GraphIndex(HashMap<Option<NodeId>, Vec<InternedTripleId>>);

impl GraphIndex {
    /// Create a new `GraphIndex`
    pub(crate) fn new() -> GraphIndex {
        GraphIndex(HashMap::new())
    }

    /// Add an `InternedTripeId` to the provided `Graph`'s `NodeId`.
    pub(crate) fn add_triple(
        &mut self, graph_id: Option<NodeId>, triple_id: InternedTripleId
    ) {
        self.0.entry(graph_id).or_insert(vec![])
            .push(triple_id);
    }
}

impl Deref for GraphIndex {
    type Target = HashMap<Option<NodeId>, Vec<InternedTripleId>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GraphIndex {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}