use std::ops::Deref;

use crate::FastIndexSet;
use crate::graphs::GraphId;
use crate::nodes::{raw::InternedNode, NodeId};

use super::Triple;

/// A [`Quad`] is a [`Triple`] with an optional [`GraphId`] to assign it to a 
/// [`Graph`] that has been registered with a 
/// [`TripleStore`](super::triples::TripleStore).
#[derive(Debug)]
pub struct Quad {
    graph: GraphId,
    triple: Triple
}

impl Quad {
    /// Create a new `Quad`.
    pub fn new(
        graph: GraphId, triple: Triple
    ) -> Quad {
        Quad { graph, triple }
    }

    /// Consume this `Quad` and splits it into a tuple of its (`GraphId`, 
    /// `Triple`).
    pub fn into_parts(self) -> (GraphId, Triple) {
        (self.graph, self.triple)
    }
}


#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedQuad {
    graph: GraphId,
    subject: NodeId,
    predicate: NodeId,
    object: NodeId
}

impl InternedQuad {
    /// Create a new [`InternedQuad`].
    pub fn new(
        graph: GraphId, subject: NodeId,
        predicate: NodeId, object: NodeId
    ) -> InternedQuad {
        InternedQuad { graph, subject, predicate, object }
    }

    /// Get the `NodeId` for the `subject` node.
    pub fn subject_id(&self) -> NodeId {
        self.subject
    }

    /// Get the `NodeId` for the `predicate` node.
    pub fn predicate_id(&self) -> NodeId {
        self.predicate
    }

    /// Get the `NodeId` for the `object` node.
    pub fn object_id(&self) -> NodeId {
        self.object
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct QuadId(u32);

impl QuadId {
    pub fn from(ix: usize) -> QuadId {
        debug_assert!(ix <= u32::MAX as usize);
        QuadId(ix as u32)
    }
}

impl Deref for QuadId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub(crate) struct QuadStore {
    store: FastIndexSet<InternedQuad>
}

impl QuadStore {
    /// Create a new [`QuadStore`].
    pub(crate) fn new() -> QuadStore {
        QuadStore {
            store: FastIndexSet::default()
        }
    }

    /// Add a [`Quad`] to this `QuadStore`, returning the `Quad`'s index as a 
    /// `QuadId`.
    pub(crate) fn intern_quad(&mut self, quad: InternedQuad) -> QuadId {
        QuadId::from(self.store.insert_full(quad).0)
    }

    /// For the given `GraphId`, return a `Vec<(NodeId, NodeId, NodeId)>` for 
    /// the interned `subject`, `predicate` and `object`.
    pub(crate) fn query_nodes_by_graph(
        &self, graph_id: GraphId
    ) -> Vec<(NodeId, NodeId, NodeId)> {
        self.store.iter()
            .filter(|q| q.graph == graph_id)
            .map(|q| (q.subject_id(), q.predicate_id(), q.predicate_id()))
            .collect()
    }
}

/// A [`QuadView`] is a view on the built elements of a [`Quad`], via an 
/// [`InternedQuad`]. It provides references to 
#[derive(Debug)]
pub(crate) struct QuadView<'a> {
    graph_namespace_prefix: &'a str,
    graph_endpoint: &'a str,
    subject: &'a InternedNode,
    predicate: &'a InternedNode,
    object: &'a InternedNode
}

impl<'a> QuadView<'a> {
    pub(crate) fn new(
        graph_namespace_prefix: &'a str, graph_endpoint: &'a str,
        subject: &'a InternedNode, predicate: &'a InternedNode, 
        object: &'a InternedNode
    ) -> QuadView<'a> {
        QuadView {
            graph_namespace_prefix, graph_endpoint, subject, predicate, object
        }
    }
}