mod store;
mod view;

pub(crate) use store::{InternedTripleId, InternedTripleStore};
pub(crate) use view::TripleView;

use crate::nodes::{Graph, NodeId, Object, Predicate, Subject};

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
    pub fn new(
        subject: Subject<'a>, predicate: Predicate<'a>, object: Object<'a>
    ) -> Triple<'a> {
        Triple {
            graph: None,
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into()
        }
    }

    /// Create a new `Triple` from parts, with a defined graph.
    pub fn new_with_graph(
        graph: Graph<'a>, subject: Subject<'a>, predicate: Predicate<'a>, object: Object<'a>
    ) -> Triple<'a> {
        Triple {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph: Some(graph.into()),
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
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    pub(crate) fn subject(&self) -> NodeId {
        self.subject
    }

    /// Get a reference to the `predicate` `NodeId`.
    pub(crate) fn predicate(&self) -> NodeId {
        self.predicate
    }

    /// Get a reference to the `object` `NodeId`.
    pub(crate) fn object(&self) -> NodeId {
        self.object
    }
}