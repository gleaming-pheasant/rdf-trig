use std::io::{self, Write};

use crate::graphs::{Graph, GraphId, GraphStore, InternedGraph};
use crate::namespaces::{Namespace, NamespaceId, NamespaceStore};
use crate::nodes::{
    InternedIriNode,
    InternedNode,
    NodeId,
    NodeStore,
    Object,
    Predicate,
    Subject
};
use crate::groups::quads::{InternedQuad, Quad, QuadId, QuadStore, QuadView};
use crate::groups::triples::{InternedTriple, Triple, TripleId, TripleStore};
use crate::traits::{IntoTriple, IntoTriples, WriteTriG};

/// A [`DataStore`] should be the main entry point for applications using this 
/// crate.
/// 
/// Create a `DataStore`, and use it to register `Graph`s (for quick dynamic 
/// declaration of [`Quad`]s, without the need to clone each `Graph`'s IRI per 
/// `Quad`) and [`Triple`]s.
pub struct DataStore {
    namespaces: NamespaceStore,
    graphs: GraphStore,
    nodes: NodeStore,
    quads: QuadStore,
    triples: TripleStore
}

impl DataStore {
    /// Create a new [`DataStore`].
    pub fn new() -> DataStore {
        DataStore {
            namespaces: NamespaceStore::new(),
            graphs: GraphStore::new(),
            nodes: NodeStore::new(),
            quads: QuadStore::new(),
            triples: TripleStore::new()
        }
    }

    /// Add a [`Graph`] to the collection of `Graph`s in this `DataStore`. Doing 
    /// so will return a [`GraphId`] which can be used to assign [`Triple`]s to 
    /// a graph (converting them to [`Quad`]s).
    pub fn add_graph(&mut self, graph: Graph) -> GraphId {
        let (ns, ep) = graph.into_parts();
        let ns_id = self.namespaces.intern_namespace(ns);
        
        self.graphs.intern_graph(InternedGraph::new(ns_id, ep))
    }

    /// Add a `Triple` (or implementor of [`IntoTriple`]) to this `DataStore`.
    pub fn add_triple<T: IntoTriple>(&mut self, triple: T) {
        let (sub_id, pred_id, obj_id) = self.intern_nodes_from_triple(triple.into_triple());
        self.triples.intern_triple(InternedTriple::new(sub_id, pred_id, obj_id));
    }

    /// Add all of the `Triple`s from an impl [`IntoTriples`] iterator to this 
    /// `DataStore`.
    pub fn add_triples<T: IntoTriples>(&mut self, triples: T) {
        for triple in triples.into_triples() {
            self.add_triple(triple);
        }
    }

    /// Assign the provided `Triple` (or implementor of `Into<Triple>`) to the 
    /// provided `GraphId` and add it to this `DataStore`.
    pub fn add_triple_to_graph<T: IntoTriple>(
        &mut self, graph_id: GraphId, triple: T
    ) {
        let (sub_id, pred_id, obj_id) = self.intern_nodes_from_triple(
            triple.into_triple()
        );

        self.quads.intern_quad(InternedQuad::new(
            graph_id, sub_id, pred_id, obj_id
        ));
    }

    /// Assign the provided `Vec<Triple>` (or implementor of `Into<Vec<Triple>>`) 
    /// to the provided `GraphId` and add them to this `DataStore`.
    pub fn add_triples_to_graph<T: IntoTriples>(
        &mut self, graph_id: GraphId, triples: T
    ) {
        for triple in triples.into_triples() {
            let (sub_id, pred_id, obj_id) = self.intern_nodes_from_triple(triple);
            self.quads.intern_quad(InternedQuad::new(
                graph_id, sub_id, pred_id, obj_id
            ));
        }
    }

    /// Add a pre-built [`Quad`] (with a [`GraphId`] already registered to a 
    /// `Triple`) to this `DataStore`.
    pub fn add_quad(&mut self, quad: Quad) {
        let (graph_id, triple) = quad.into_parts();
        let (sub_id, pred_id, obj_id) = self.intern_nodes_from_triple(triple);

        self.quads.intern_quad(InternedQuad::new(
            graph_id, sub_id, pred_id, obj_id
        ));
    }

    /// Get a full list of `Quad`s for the given `GraphId`.
    /// 
    /// A `Quad` is automatically declared whenever you call 
    /// [Self::add_quad], [Self::add_triple_to_graph] or 
    /// [Self::add_triples_to_graph].
    pub fn query_quads_by_graph(&self, graph_id: GraphId) -> Vec<QuadView> {
        let graph_nodes = self.quads.query_nodes_by_graph(graph_id);
        let mut quad_views: Vec<QuadView> = Vec::with_capacity(graph_nodes.len());

        for (sub_id, pred_id, obj_id) in graph_nodes {
            if let InternedNode::Iri(iri) = self.query_node(sub_id) {
                let sub_namespace = self.query_namespace(iri.namespace_id());
            }
        }

        todo!()

        // QuadView::new(
        //     self.query_graph_namespace_prefix(graph_id),
        //     self.query_graph_endpoint(graph_id),
        //     subject,
        //     predicate,
        //     object
        // )
    }

    /// Retrieve all triples 
    fn retrieve_all_triples() -> () {
        todo!()
    }
    
    /// Add a [`Namespace`] to the `namespaces` [`NamespaceStore`] returning its 
    /// [`NamespaceId`] (index in the store, cast as u32).
    fn intern_namespace(&mut self, namespace: Namespace) -> NamespaceId {
        self.namespaces.intern_namespace(namespace)
    }

    /// Add an [`InternedNode`] to the `nodes` [`NodeStore`] returning its 
    /// [`NodeId`] (index in the store, cast as u32).
    fn intern_node(&mut self, node: InternedNode) -> NodeId {
        self.nodes.intern_node(node)
    }

    /// Add an [`InternedQuad`] to the `quads` [`QuadStore`] returning its 
    /// [`QuadId`] (index in the store, cast as u32).
    fn intern_quad(&mut self, quad: InternedQuad) -> QuadId {
        self.quads.intern_quad(quad)
    }

    /// Add an [`InternedTriple`] to the `triples` [`TripleStore`] returning its 
    /// [`TripleId`] (index in the store, cast as u32).
    fn intern_triple(&mut self, triple: InternedTriple) -> TripleId {
        self.triples.intern_triple(triple)
    }

    /// Split a [`Triple`] into parts, intern each of the `Triple`'s "node" in 
    /// the `nodes` [`NodeStore`], and return a (`NodeId`, `NodeId`, `NodeId`) 
    /// tuple of the [`NodeId`] for the `subject`, `predicate` and `object`.
    fn intern_nodes_from_triple(
        &mut self, triple: Triple
    ) -> (NodeId, NodeId, NodeId) {
        let (subject, predicate, object) = triple.into_parts();
        
        let interned_subject = match subject {
            Subject::Blank(blank) => InternedNode::Blank(blank),
            Subject::Iri(iri) => {
                let (namespace, endpoint) = iri.into_parts();
                let namespace_id = self.intern_namespace(namespace);
                InternedNode::Iri(InternedIriNode::new(namespace_id, endpoint))
            }
        };

        let interned_predicate = match predicate {
            Predicate::Iri(iri) => {
                let (namespace, endpoint) = iri.into_parts();
                let namespace_id = self.intern_namespace(namespace);
                InternedNode::Iri(InternedIriNode::new(namespace_id, endpoint))
            }
        };

        let interned_object = match object {
            Object::Blank(blank) => InternedNode::Blank(blank),
            Object::Iri(iri) => {
                let (namespace, endpoint) = iri.into_parts();
                let namespace_id = self.intern_namespace(namespace);
                InternedNode::Iri(InternedIriNode::new(namespace_id, endpoint))
            },
            Object::Literal(literal) => InternedNode::Literal(literal)
        };

        (
            self.intern_node(interned_subject),
            self.intern_node(interned_predicate),
            self.intern_node(interned_object)
        )
    }

    /// Retrieve a reference to a `Namespace` from the provided `NamespaceId`.
    fn query_namespace(&self, ns_id: NamespaceId) -> &Namespace {
        self.namespaces.query_namespace(ns_id)
    }

    /// Retrieve the `endpoint` for a `Graph` from the provided `GraphId`.
    fn query_graph_endpoint(&self, graph_id: GraphId) -> &str {
        self.graphs.query_endpoint(graph_id)
    }

    /// Retrieve a `Graph`s `Namespace` from the provided `GraphId`.
    fn query_graph_namespace(&self, graph_id: GraphId) -> NamespaceId {
        self.graphs.query_namespace(graph_id)
    }

    /// Retrieve a `Graph`s `Namespace`'s `prefix` from the provided `GraphId`.
    fn query_graph_namespace_prefix(&self, graph_id: GraphId) -> &str {
        self.namespaces.query_namespace(
            self.query_graph_namespace(graph_id)
        ).prefix()
    }

    /// Retrieve an `InternedNode` from a provided `NodeId`.
    fn query_node(&self, node_id: NodeId) -> &InternedNode {
        self.nodes.query_node(node_id)
    }
}

impl WriteTriG for DataStore {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.namespaces.write_trig(writer)?;

        Ok(())
    }
}