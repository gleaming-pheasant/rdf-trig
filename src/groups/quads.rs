use std::ops::Deref;

use crate::FastIndexSet;
use crate::graphs::GraphId;
use crate::groups::triples::Triple;
use crate::nodes::NodeId;

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
}

impl<'a> IntoIterator for &'a QuadStore {
    type Item = &'a InternedQuad;
    type IntoIter = indexmap::set::Iter<'a, InternedQuad>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.store.iter()
    }
}