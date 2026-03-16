use std::ops::Deref;

use crate::FastIndexSet;
use crate::nodes::{
    Graph,
    NodeId,
    Object,
    Predicate,
    Subject
};

/// A `Triple` is wrapper around the three constituent parts of an RDF triple: 
/// [`Subject`], [`Predicate`] and [`Object`], as well as an optional [`Graph`].
#[derive(Clone, Debug)]
pub struct Triple<'a> {
    graph: Option<Graph<'a>>,
    subject: Subject<'a>,
    predicate: Predicate<'a>,
    object: Object<'a>
}

impl<'a> Triple<'a> {
    /// Create a new `Triple` from parts. This triple will automatically be 
    /// assigned to the default graph.
    pub fn new<S,P,O>(subject: S, predicate: P, object: O) -> Triple<'a>
    where
        S: Into<Subject<'a>>,
        P: Into<Predicate<'a>>,
        O: Into<Object<'a>>
    {
        Triple {
            graph: None,
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into()
        }
    }

    /// Create a new `Triple` from parts, with a defined graph.
    pub fn new_with_graph<G,S,P,O>(
        graph: G, subject: S, predicate: P, object: O
    ) -> Triple<'a>
    where
        G: Into<Graph<'a>>,
        S: Into<Subject<'a>>,
        P: Into<Predicate<'a>>,
        O: Into<Object<'a>>
    {
        Triple {
            graph: Some(graph.into()),
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into()
        }
    }

    /// Consume this `Triple`, returning a tuple of the contained ([`Graph`], 
    /// [`Subject`], [`Predicate`] and [`Object`]).
    pub(crate) fn into_parts(self)
    -> (Option<Graph<'a>>, Subject<'a>, Predicate<'a>, Object<'a>) {
        (self.graph, self.subject, self.predicate, self.object)
    }
}

/// An [`InternedTriple`] is a struct to be built from the interned `nodes` 
/// which make up a regular [`Triple`].
/// 
/// It takes a [`NodeId`] for each of the `subject`, `predicate` and `object`and 
/// effectively takes advantage of zero-cost abstraction to serve as a labelled 
/// tuple over already-interned nodes.
/* The use of Option here is one of the few places where performance hasn't been 
top priority. Using a "DefaultGraph" with a pre-indexed Id of 0 would be more 
performant than not wrapping `Node`s in `Option`s based purely on memory 
overhead, of wrapping it in the enum, but it made for a very disjointed, hard to 
maintain crate. */
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedTriple {
    graph: Option<NodeId>,
    subject: NodeId,
    predicate: NodeId,
    object: NodeId
}

impl InternedTriple {
    /// Create a new `InternedTriple` from a collection of interned `NodeId`s.
    pub(crate) fn new(
        graph: Option<NodeId>, subject: NodeId, predicate: NodeId, object: NodeId
    ) -> InternedTriple {
        InternedTriple { graph, subject, predicate, object }
    }

    /// Get a reference to the `graph` `Option<NodeId>`.
    pub(crate) fn graph(&self) -> Option<NodeId> {
        self.graph
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

    /// Retrieve an `InternedTriple` reference from the provided 
    /// `InternedTripleId`.
    /// 
    /// Use of [`Option::unwrap`] is considered safe in this function, as the 
    /// crate only allows the generation of `InternedTripleId`s is only through 
    /// [`Self::intern_triple`].
    /// 
    /// Any future functionality that allows removal of items from `...Store`s 
    /// must address this.
    pub(crate) fn query_triple(
        &self, triple_id: InternedTripleId
    ) -> &InternedTriple {
        self.0.get_index(*triple_id as usize).unwrap()
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