mod views;

use std::io::{Result as IoResult, Write};

use crate::namespaces::{Namespace, NamespaceId, NamespaceStore};
use crate::namespaces::statics::XSD;
use crate::nodes::{NodeId, NodeStore, Object, Subject};
use crate::quads::{InternedQuad, Quad, QuadId, QuadStore};
use crate::triples::{
    InternedTriple,
    Triple,
    TripleId,
    TripleStore
};
use crate::traits::{IntoTriple, WriteTriG};

/// A `DataStore` should be the main entry point for applications using this 
/// crate.
/// 
/// Create a `DataStore`, and use it to register `Graph`s (for quick dynamic 
/// declaration of [`Quad`]s, without the need to clone each `Graph`'s IRI per 
/// `Quad`) and [`Triple`]s.
/// 
/// By default, a `DataStore` will be initialised with the [`XSD`] `Namespace` 
/// already initialised, to allow for the safe use of "literal" nodes.
#[derive(Debug)]
pub struct DataStore {
    namespaces: NamespaceStore,
    nodes: NodeStore,
    quads: QuadStore,
    triples: TripleStore
}

impl DataStore {
    /// Create a new [`DataStore`].
    pub fn new() -> DataStore {
        let mut namespaces = NamespaceStore::new();
        namespaces.intern_namespace(XSD);

        DataStore {
            namespaces,
            nodes: NodeStore::new(),
            quads: QuadStore::new(),
            triples: TripleStore::new()
        }
    }

    /// Add a `Triple` (or implementor of [`IntoTriple`]) to this `DataStore`.
    pub fn add_triple<T: IntoTriple>(&mut self, triple: T) {
        let (sub_id, pred_id, obj_id) = self.intern_nodes_from_triple(triple.into_triple());
        self.intern_triple(InternedTriple::new(sub_id, pred_id, obj_id));
    }

    /// Add a pre-built [`Quad`] (with a [`GraphId`] already registered to a 
    /// `Triple`) to this `DataStore`.
    pub fn add_quad(&mut self, quad: Quad) {
        let (graph_id, triple) = quad.into_parts();
        let (sub_id, pred_id, obj_id) = self.intern_nodes_from_triple(triple);

        self.intern_quad(InternedQuad::new(
            graph_id, sub_id, pred_id, obj_id
        ));
    }

    /// Retrieve a [`FullGraphView`] of a [`Graph`] for the provided [`GraphId`].
    pub fn get_full_graph_view(&self, graph_id: GraphId) -> FullGraphView<'_> {
        let mut fgv = FullGraphView::new(self.get_graph_view(graph_id));

        for quad in self.quads.iter() {
                fgv.add_triple_view(
                    TripleView::new(
                        self.node_id_to_view(quad.subject_id()),
                        self.node_id_to_view(quad.predicate_id()),
                        self.node_id_to_view(quad.object_id())
                    )
                );
            }

            fgv
    }

    /// Retrieve a [`GraphView`] of a [`Graph`] for the provided [`GraphId`].
    pub fn get_graph_view(&self, graph_id: GraphId) -> GraphView<'_> {
        GraphView::new(
            self.namespaces.query_namespace(self.graphs.query_namespace(graph_id)),
            self.graphs.query_endpoint(graph_id)
        )
    }

    /// Retrieve an iterator over [`FullGraphView`]s for every [`Graph`] 
    /// contained in this `DataStore`.
    pub fn get_all_full_graph_views(&self)
    -> impl Iterator<Item = FullGraphView<'_>> {
        (0..self.graphs.len()).map(|ix| {
            self.get_full_graph_view(GraphId::from(ix))
        })
    }

    /// Retrieve all `Triple`s contained in this `DataStore` as an iterator over 
    /// [`TripleView`]s.
    pub fn all_triples(&self) -> impl Iterator<Item = TripleView<'_>> {
        self.triples.iter()
            .map(|trip| {
                TripleView::new(
                    self.node_id_to_view(*trip.subject()),
                    self.node_id_to_view(*trip.predicate()),
                    self.node_id_to_view(*trip.object())
            )})
    }

    /// Private function which takes the provided `NodeId`, and returns a 
    /// [`NodeView`], expanding an `InternedIriNode` with a namespace if present.
    #[inline]
    fn node_id_to_view(&self, node_id: NodeId) -> NodeView<'_> {
        match self.nodes.query_node(node_id) {
            InternedNode::Blank(blank) => NodeView::Blank(blank),
            InternedNode::Iri(iri) => {
                NodeView::Iri(IriNodeView::new(
                    self.namespaces.query_namespace(iri.namespace_id()),
                    iri.endpoint()
                ))
            },
            InternedNode::Literal(literal) => NodeView::Literal(literal)
        }
    }
    
    /// Add a [`Namespace`] to the `namespaces` [`NamespaceStore`] returning its 
    /// [`NamespaceId`] (index in the store, cast as u32).
    #[inline]
    fn intern_namespace(&mut self, namespace: Namespace) -> NamespaceId {
        self.namespaces.intern_namespace(namespace)
    }

    /// Add an [`InternedNode`] to the `nodes` [`NodeStore`] returning its 
    /// [`NodeId`] (index in the store, cast as u32).
    #[inline]
    fn intern_node(&mut self, node: InternedNode) -> NodeId {
        self.nodes.intern_node(node)
    }

    /// Add an [`InternedQuad`] to the `quads` [`QuadStore`] returning its 
    /// [`QuadId`] (index in the store, cast as u32).
    #[inline]
    fn intern_quad(&mut self, quad: InternedQuad) -> QuadId {
        self.quads.intern_quad(quad)
    }

    /// Add an [`InternedTriple`] to the `triples` [`TripleStore`] returning its 
    /// [`TripleId`] (index in the store, cast as u32).
    #[inline]
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

        let interned_predicate = {
            let (namespace, endpoint) = predicate.into_parts();
            let namespace_id = self.intern_namespace(namespace);
            InternedNode::Iri(InternedIriNode::new(namespace_id, endpoint))
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
}

impl<'a> WriteTriG for DataStore {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        self.namespaces.write_trig(writer)?;
        writer.write_all(b"\n")?;

        for trip in self.all_triples() {
            trip.write_trig(writer)?;
            writer.write_all(b"\n")?;
        }

        for graph in self.get_all_full_graph_views() {
            graph.write_trig(writer)?;
            writer.write_all(b"\n")?;
        }
        
        Ok(())
    }
}