use std::ops::Deref;

use crate::FastIndexSet;
use crate::graphs::GraphId;
use crate::nodes::{NodeId, Object, Predicate, Subject};
use crate::traits::IntoTriple;

use super::Quad;

#[derive(Debug)]
pub struct Triple {
    subject: Subject,
    predicate: Predicate,
    object: Object
}

impl Triple {
    /// Create a new [`Triple`] from parts.
    pub fn new(subject: Subject, object: Object, predicate: Predicate) -> Triple {
        Triple { subject, predicate, object }
    }

    /// Convert this `Triple` into a [`Quad`] by assigning it with the provided 
    /// [`GraphId`].
    pub fn into_quad(self, graph: GraphId) -> Quad {
        Quad::new(graph, self)
    }

    /// Consume this `Triple`, returning a tuple of the contained ([`Subject`], 
    /// [`Predicate`] and [`Object`])
    pub fn into_parts(self) -> (Subject, Predicate, Object) {
        (self.subject, self.predicate, self.object)
    }
}

impl IntoTriple for Triple {
    #[inline(always)]
    fn into_triple(self) -> Triple {
        self
    }
}


/// An [`InternedTriple`] is a struct to be built from the interned `nodes` 
/// which make up a regular [`Triple`].
/// 
/// It takes a [`NodeId`] for each of the `subject`, `predicate` and `object`.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedTriple {
    subject: NodeId,
    predicate: NodeId,
    object: NodeId
}

impl InternedTriple {
    /// Create a new `InternedTriple` from a collection of interned `NodeId`s.
    pub fn new(
        sub_id: NodeId, pred_id: NodeId, obj_id: NodeId
    ) -> InternedTriple {
        InternedTriple { subject: sub_id, predicate: pred_id, object: obj_id }
    }
}

pub(crate) struct TripleStore {
    store: FastIndexSet<InternedTriple>
}

impl TripleStore {
    /// Create a new [`TripleStore`].
    pub(crate) fn new() -> TripleStore {
        TripleStore {
            store: FastIndexSet::default()
        }
    }

    /// Add an [`InternedTriple`] to the `TripleStore` returning a [`TripleId`].
    pub(crate) fn intern_triple(&mut self, triple: InternedTriple) -> TripleId {
        TripleId::from(self.store.insert_full(triple).0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct TripleId(u32);

impl TripleId {
    pub(crate) fn from(ix: usize) -> TripleId {
        debug_assert!(ix <= u32::MAX as usize);
        TripleId(ix as u32)
    }
}

impl Deref for TripleId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}