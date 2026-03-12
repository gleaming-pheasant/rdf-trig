mod index;
mod views;

use std::io::{Result as IoResult, Write};

use crate::namespaces::Namespace;
use crate::namespaces::statics::XSD;
use crate::namespaces::store::{NamespaceId, NamespaceStore};
use crate::nodes::{NodeId, NodeStore, Object, Predicate, Subject};
use crate::triples::{
    InternedTriple,
    Triple,
    InternedTripleStore
};
use crate::traits::WriteTriG;
use crate::triplestore::index::GraphIndex;

/// A `TripleStore` should be the main entry point for applications using this 
/// crate.
/// 
/// Create a `TripleStore`, and use it to register `Graph`s (for quick dynamic 
/// declaration of [`Quad`]s, without the need to clone each `Graph`'s IRI per 
/// `Quad`) and [`Triple`]s.
#[derive(Debug)]
pub struct TripleStore {
    namespaces: NamespaceStore,
    nodes: NodeStore,
    triples: InternedTripleStore,
    graph_index: GraphIndex
}

impl TripleStore {
    /// Create a new `TripleStore`.
    /// 
    // Initialises the triplestore with the [`XSD`] namespace already 
    // registered, due to its necessity for using any literal types.
    pub fn new() -> TripleStore {
        let mut namespaces = NamespaceStore::new();
        namespaces.intern_namespace(XSD);

        TripleStore {
            namespaces,
            nodes: NodeStore::new(),
            triples: InternedTripleStore::new(),
            graph_index: GraphIndex::new()
        }
    }

    /// Add a `Triple` (or impl [`IntoTriple`]) to this `TripleStore`.
    pub fn add_triple<'a, T: Into<Triple<'a>>>(&mut self, triple: T) {
        let triple = self.intern_nodes_from_triple(triple.into());
        
        // Get a clone of the `NodeId` for the graph for adding to the index.
        let graph_id = triple.graph();

        let interned_triple_id = self.triples.intern_triple(triple);

        // Add to index.
        self.graph_index.add_triple(graph_id, interned_triple_id);
    }

    // /// Retrieve a [`FullGraphView`] of a [`Graph`] for the provided [`GraphId`].
    // pub fn get_full_graph_view(&self, graph_id: GraphId) -> FullGraphView<'_> {
    //     let mut fgv = FullGraphView::new(self.get_graph_view(graph_id));

    //     for quad in self.quads.iter() {
    //             fgv.add_triple_view(
    //                 TripleView::new(
    //                     self.node_id_to_view(quad.subject_id()),
    //                     self.node_id_to_view(quad.predicate_id()),
    //                     self.node_id_to_view(quad.object_id())
    //                 )
    //             );
    //         }

    //         fgv
    // }

    /// Retrieve a [`GraphView`] of a [`Graph`] for the provided [`GraphId`].
    // pub fn get_graph_view(&self, graph_id: GraphId) -> GraphView<'_> {
    //     GraphView::new(
    //         self.namespaces.query_namespace(self.graphs.query_namespace(graph_id)),
    //         self.graphs.query_local_name(graph_id)
    //     )
    // }

    /// Retrieve an iterator over [`FullGraphView`]s for every [`Graph`] 
    /// contained in this `TripleStore`.
    // pub fn get_all_full_graph_views(&self)
    // -> impl Iterator<Item = FullGraphView<'_>> {
    //     (0..self.graphs.len()).map(|ix| {
    //         self.get_full_graph_view(GraphId::from(ix))
    //     })
    // }

    /// Retrieve all `Triple`s contained in this `TripleStore` as an iterator over 
    /// [`TripleView`]s.
    // pub fn all_triples(&self) -> impl Iterator<Item = TripleView<'_>> {
    //     self.triples.iter()
    //         .map(|trip| {
    //             TripleView::new(
    //                 self.node_id_to_view(*trip.subject()),
    //                 self.node_id_to_view(*trip.predicate()),
    //                 self.node_id_to_view(*trip.object())
    //         )})
    // }

    /// Private function which takes the provided `NodeId`, and returns a 
    /// [`NodeView`], expanding an `InternedIriNode` with a namespace if present.
    // #[inline]
    // fn node_id_to_view(&self, node_id: NodeId) -> NodeView<'_> {
    //     match self.nodes.query_node(node_id) {
    //         InternedNode::Blank(blank) => NodeView::Blank(blank),
    //         InternedNode::Iri(iri) => {
    //             NodeView::Iri(IriNodeView::new(
    //                 self.namespaces.query_namespace(iri.namespace_id()),
    //                 iri.local_name()
    //             ))
    //         },
    //         InternedNode::Literal(literal) => NodeView::Literal(literal)
    //     }
    // }
    
    /// Add a [`Namespace`] to the `namespaces` [`NamespaceStore`] returning its 
    /// [`NamespaceId`] (index in the store, cast as u32).
    #[inline]
    fn intern_namespace(&mut self, namespace: Namespace) -> NamespaceId {
        self.namespaces.intern_namespace(namespace)
    }

    /// Split a [`Triple`] into parts, intern each of the `Triple`'s "node" in 
    /// the `nodes` [`NodeStore`], and return an [`InternedTriple`] for 
    /// interning in this `TripleStore`'s [`InternedTripleStore`].
    fn intern_nodes_from_triple(
        &mut self, triple: Triple
    ) -> InternedTriple {
        let (graph, subject, predicate, object) = triple.into_parts();
        
        let graph_id = match graph {
            Some(graph) => Some(
                self.nodes.intern_node(
                    graph.0.into_staging(&mut self.namespaces))
                ),
            None => None
        };

        let subject_id = self.intern_subject(subject);
        let predicate_id = self.intern_predicate(predicate);
        let object_id = self.intern_object(object);

        InternedTriple::new(graph_id, subject_id, predicate_id, object_id)
    }

    /// Interns a `Subject` and returns its `NodeId`.
    fn intern_subject<'a>(&mut self, subject: Subject<'a>) -> NodeId {
        match subject {
            Subject::Blank(blank) => self.nodes.intern_node(blank.into()),
            Subject::Iri(iri) => self.nodes.intern_node(
                iri.into_staging(&mut self.namespaces)
            )
        }
    }

    /// Interns a `Predicate` and returns its `NodeId`.
    fn intern_predicate<'a>(&mut self, predicate: Predicate<'a>) -> NodeId {
        self.nodes.intern_node(
            predicate.0.into_staging(&mut self.namespaces)
        )
    }

    /// Interns an `object` and returns its `NodeId`.
    fn intern_object<'a>(&mut self, object: Object<'a>) -> NodeId {
        match object {
            Object::Blank(blank) => self.nodes.intern_node(blank.into()),
            Object::Iri(iri) => self.nodes.intern_node(
                iri.into_staging(&mut self.namespaces)
            ),
            Object::Literal(literal) => self.nodes.intern_node(literal.into())
        }
    }
}

impl<'a> WriteTriG for TripleStore {
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