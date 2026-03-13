mod index;
mod views;

use std::collections::hash_map::Iter;
use std::io::{Result as IoResult, Write};

use crate::namespaces::Namespace;
use crate::namespaces::statics::XSD;
use crate::namespaces::store::{NamespaceId, NamespaceStore};
use crate::nodes::{NodeId, NodeStore, StagingNode};
use crate::triples::{
    InternedTriple, InternedTripleId, InternedTripleStore, Triple
};
use crate::traits::WriteTriG;
use crate::triplestore::index::GraphIndex;
use crate::triplestore::views::{IriNodeView, NodeView, TripleView};

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

    /// Add all of the "nodes" in the provided `Triple` to this `TripleStore`'s 
    /// [`NodeStore`], returning an [`InternedTriple`] wrapper around the 
    /// [`NodeID`] for each "node".
    fn intern_nodes_from_triple(
        &mut self, triple: Triple<'_>
    ) -> InternedTriple {
        let (graph, subject, predicate, object) = triple.into_parts();

        
    }

    /// Retrieve a [`NodeView`] for the given [`NodeId`].
    /// 
    /// This function resolves any contained [`Namespace`]s if the `Node` is an 
    /// IRI node.
    fn resolve_node<'a>(&'a self, node_id: NodeId) -> NodeView<'a> {
        match self.nodes.query_node(node_id) {
            StagingNode::Blank(blank) => NodeView::Blank(blank),
            StagingNode::Iri(iri) => {
                let namespace = self.resolve_namespace(iri.namespace_id());
                NodeView::Iri(IriNodeView::new(namespace.prefix(), iri.local_name()))
            },
            StagingNode::Literal(literal) => NodeView::Literal(literal)
        }
    }

    /// Retrieve a reference to a [`Namespace`] for the given [`NamespaceId`].
    fn resolve_namespace(
        &self, namespace_id: NamespaceId
    ) -> &Namespace {
        self.namespaces.query_namespace(namespace_id)
    }

    /// Retrieve a [`TripleView`] for the given [`InternedTripleId`].
    /// 
    /// This function resolves any contained [`Namespace`]s for any IRI Nodes.
    fn resolve_triple<'a>(
        &'a self, triple_id: InternedTripleId
    ) -> TripleView<'a> {
        let interned_triple = self.triples.query_triple(triple_id);

        TripleView::new(
            self.resolve_node(*interned_triple.subject()),
            self.resolve_node(*interned_triple.predicate()),
            self.resolve_node(*interned_triple.object())
        )
    }

    /// Retrieve an iterator of all graphs and their corresponding 
    /// `InternedTripleId`s in the `graph_index` of this `TripleStore`.
    fn graphs_iter<'a>(&'a self)
    -> Iter<'a, Option<NodeId>, Vec<InternedTripleId>> {
        self.graph_index.iter()
    }
}

impl WriteTriG for TripleStore {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        self.namespaces.write_trig(writer)?;
        writer.write_all(b"\n")?;

        for graph in self.graphs_iter() {
            match graph.0 {
                None => {
                    for triple_id in &*graph.1 {
                        self.resolve_triple(*triple_id).write_trig(writer)?;
                        writer.write_all(b"\n")?;
                    }
                },
                Some(graph_id) => {
                    // Write the opening for the graph first.
                    self.resolve_node(*graph_id).write_trig(writer)?;
                    writer.write_all(b" { ")?; // Padding before first triple.

                    // We don't care about writing white space to align with the 
                    // graph declaration, as often stylised. This is a waste of 
                    // bytes.
                    for triple_id in &*graph.1 {
                        self.resolve_triple(*triple_id).write_trig(writer)?;
                        writer.write_all(b"\n")?;
                    }

                    writer.write_all(b" }\n")?; // Close the graph block
                }
            }
        }
        
        Ok(())
    }
}