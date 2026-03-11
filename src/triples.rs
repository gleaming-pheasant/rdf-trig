use std::ops::Deref;

use crate::FastIndexSet;
use crate::nodes::{IriNode, NodeId, Object, Predicate, Subject};

use super::Quad;

/// A `Triple` is wrapper around the three constituent parts of an RDF triple: 
/// [`Subject`], [`Predicate`] and [`Object`].
#[derive(Debug)]
pub struct Triple<'a> {
    subject: Subject<'a>,
    predicate: Predicate<'a>,
    object: Object<'a>
}

impl<'a> Triple<'a> {
    /// Create a new `Triple` from parts.
    pub fn new(
        subject: Subject<'a>, predicate: Predicate<'a>, object: Object<'a>
    ) -> Triple<'a> {
        Triple { subject, predicate, object }
    }

    /// Convert this `Triple` into a [`Quad`] by assigning it to the graph 
    /// with the provided [`IriNode`] graph.
    pub fn into_quad(self, graph: IriNode<'a>) -> Quad<'a> {
        Quad::new(graph, self)
    }

    /// Consume this `Triple`, returning a tuple of the contained ([`Subject`], 
    /// [`Predicate`] and [`Object`]).
    pub(crate) fn into_parts(self) -> (Subject<'a>, Predicate<'a>, Object<'a>) {
        (self.subject, self.predicate, self.object)
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
    pub(crate) fn new(
        sub_id: NodeId, pred_id: NodeId, obj_id: NodeId
    ) -> InternedTriple {
        InternedTriple { subject: sub_id, predicate: pred_id, object: obj_id }
    }

    /// Get a reference to the `subject` `NodeId`.
    pub(crate) fn subject(&self) -> &NodeId {
        &self.subject
    }

    /// Get a reference to the `predicate` `NodeId`.
    pub(crate) fn predicate(&self) -> &NodeId {
        &self.predicate
    }

    /// Get a reference to the `object` `NodeId`.
    pub(crate) fn object(&self) -> &NodeId {
        &self.object
    }
}

/// A wrapper around a [`FastIndexSet<InternedTriple>`] which serves to store 
/// unique "triples" and hand out [`TripleId`]s as references to the 
/// [`InternedTriple`]s.
#[derive(Debug)]
pub(crate) struct TripleStore(FastIndexSet<InternedTriple>);

impl TripleStore {
    /// Create a new [`TripleStore`].
    pub(crate) fn new() -> TripleStore {
        TripleStore(FastIndexSet::default())
    }

    /// Add an [`InternedTriple`] to the `TripleStore` returning a [`TripleId`].
    /// 
    /// As each element of a triple is a [`NodeId`], which derives [`Clone`] and 
    /// [`Copy`] for the contained [`u32`], this differs from a `NodeStore` and 
    /// `NamespaceStore` in that it does not risk allocation of any types.
    pub(crate) fn intern_triple(&mut self, triple: InternedTriple) -> TripleId {
        TripleId::from(self.0.insert_full(triple).0)
    }
}


/// A `TripleId` is a wrapper around a `u32` and is only retrievable by 
/// converting the `usize` index from an [`IndexSet`](indexmap::IndexSet) (or a 
/// [`FastIndexSet`] for the purposes of this crate).
/// 
/// This will cause the application to panic if the number of interned nodes 
/// exceeds [`u32::MAX`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct TripleId(u32);

impl TripleId {
    /// Create a new `TripleId` by casting the provided `usize` to a `u32`.
    /// 
    /// Panics if `ix` is greater than [`u32::MAX`].
    pub(crate) fn from(ix: usize) -> TripleId {
        debug_assert!(ix <= u32::MAX as usize);
        TripleId(ix as u32)
    }
}

impl Deref for TripleId {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}