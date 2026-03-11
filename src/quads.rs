use std::ops::Deref;

use crate::triples::InternedTriple;
use crate::{FastIndexSet, Triple};
use crate::nodes::{
    IriNode,
    NodeId,
    Object,
    Predicate,
    StagingIriNode,
    StagingNode,
    Subject
};

/// A [`Quad`] is a [`Triple`] with an [`IriNode`]: the IRI for the graph 
/// that the triple is assigned to.
#[derive(Debug)]
pub struct Quad<'a> {
    graph: IriNode<'a>,
    triple: Triple<'a>
}

impl<'a> Quad<'a> {
    /// Create a new `Quad`.
    pub fn new(graph: IriNode<'a>, triple: Triple<'a>) -> Quad<'a> {
        Quad { graph, triple }
    }

    /// Create a new `Quad` from the Graph [`IriNode`] and the types that form 
    /// each part of a [`Triple`].
    /// 
    /// This method does not require the `Triple` to be constructed separately 
    /// first.
    pub fn from_parts<S,P,O>(
        graph: IriNode<'a>, subject: S, predicate: P, object: O
    ) -> Quad<'a>
    where
        S: Into<Subject<'a>>,
        P: Into<Predicate<'a>>,
        O: Into<Object<'a>>
    {
        Quad {
            graph,
            triple: Triple::new(subject.into(), predicate.into(), object.into())
        }
    }

    /// Consume this `Quad` and splits it into a tuple of its (`graph`, 
    /// `triple`).
    pub fn into_graph_and_tuple(self) -> (IriNode<'a>, Triple<'a>) {
        (self.graph, self.triple)
    }
}

/// A `StagingQuad` is a wrapper around a [`StagingIriNode`] (a graph IRI for 
/// which the [`Namespace`] has already been interned) and an [`InternedTriple`] 
/// (a [`Triple`] composed of [`NodeIds`] for its interned parts).
/// 
/// This is a staging struct, which holds these values before being exchanged 
/// for 
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StagingQuad<'a> {
    graph: StagingIriNode<'a>,
    triple: InternedTriple
}

todo!()


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

impl Deref for QuadStore {
    type Target = FastIndexSet<InternedQuad>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}