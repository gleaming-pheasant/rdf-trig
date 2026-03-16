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
use crate::triplestore::views::{GraphView, IriNodeView, NodeView, TripleView};

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
        let triple = self.intern_triple(triple.into());
        
        // Get a clone of the `NodeId` for the graph for adding to the index.
        let graph_id = triple.graph();

        let interned_triple_id = self.triples.intern_triple(triple);

        // Add to index.
        self.graph_index.add_triple(graph_id, interned_triple_id);
    }

    /// Add all of the "nodes" in the provided `Triple` to this `TripleStore`'s 
    /// [`NodeStore`], returning an [`InternedTriple`] wrapper around the 
    /// [`NodeID`] for each "node".
    fn intern_triple(
        &mut self, triple: Triple<'_>
    ) -> InternedTriple {
        let ns_store = &mut self.namespaces;
        let (graph, subject, predicate, object) = triple.into_parts();

        let graph = graph.map(|g| g.into_staging_node(ns_store));
        let subject = subject.into_staging_node(ns_store);
        let predicate = predicate.into_staging_node(ns_store);
        let object = object.into_staging_node(ns_store);

        let graph = graph.map(|sg| self.intern_node(sg));
        let subject = self.intern_node(subject);
        let predicate = self.intern_node(predicate);
        let object = self.intern_node(object);

        InternedTriple::new(
            graph, subject, predicate, object
        )
    }

    /// Add a [`StagingNode`] to the [`NodeStore`] of this `TripleStore`.
    fn intern_node<'a>(&mut self, staging_node: StagingNode<'a>) -> NodeId {
        self.nodes.intern_node(staging_node)
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

    /// Retrieve a [`GraphView`] for the given [`NodeId`].
    /// 
    /// Panics if called on any `NodeId` which doesn't resolve to an `IriNode`; 
    /// don't use it on anything but a `Graph`.
    fn resolve_graph_node<'a>(&'a self, node_id: NodeId) -> GraphView<'a> {
        match self.nodes.query_node(node_id) {
            StagingNode::Iri(iri) => {
                let namespace = self.resolve_namespace(iri.namespace_id());
                GraphView::new(namespace.iri(), iri.local_name())
            },
            _ => unreachable!()
        }
    }

    /// Retrieve a reference to a [`Namespace`] for the given [`NamespaceId`].
    fn resolve_namespace(
        &self, namespace_id: NamespaceId
    ) -> &Namespace<'static> {
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
                    self.resolve_graph_node(*graph_id).write_trig(writer)?;
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

#[cfg(test)]
mod tests {
    use crate::namespaces::statics::{AOCAT, ARIADNEPLUS, RDFS};
    use crate::nodes::predicate::{RDF_TYPE};
    use crate::nodes::{IriNode, LangStringLiteral};

    use super::*;

    #[test]
    fn test_expected_triple_write_trig() {
        let mut store = TripleStore::new();

        let triple = Triple::new(
            IriNode::new(ARIADNEPLUS, "My::Class/123"),
            IriNode::new(RDFS, String::from("label")),
            LangStringLiteral::new_en("Is a\tspecial class")
        );

        store.add_triple(triple);

        let mut buf = vec![];

        store.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(
                "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n@prefix ariadneplus: <https://ariadne-infrastructure.eu/aocat/> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\nariadneplus:My\\:\\:Class\\/123 rdfs:label \"Is a\\tspecial class\"@en .\n\n"
            )
        );
    }

    #[test]
    fn test_expected_triple_and_graph_write_trig() {
        let mut store = TripleStore::new();

        let my_namespace = Namespace::new(
            "prefix", "http://www.example.com/"
        ).unwrap();

        let quad = Triple::new_with_graph(
            IriNode::new(my_namespace, "SND::Archaeology"),
            IriNode::new(ARIADNEPLUS, "StainedGlassClass"),
            IriNode::new(RDFS, String::from("label")),
            LangStringLiteral::new_en("Is a piece of stained glass")
        );

        store.add_triple(quad);

        let triple = Triple::new(
            IriNode::new(ARIADNEPLUS, "Natural History Museum"),
            RDF_TYPE,
            IriNode::new(AOCAT, "AO_Agent")
        );

        store.add_triple(triple);

        let mut buf = vec![];

        store.write_trig(&mut buf).unwrap();

        let string_output = String::from_utf8(buf).unwrap();

        // No guarantee of the output order here, hence .contains().
        assert!(string_output.contains(
            "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n"
        ));
        assert!(string_output.contains(
            "<http://www.example.com/SND::Archaeology> {"
        ));
        assert!(string_output.contains(
            "{ ariadneplus:StainedGlassClass rdfs:label \"Is a piece of stained glass\"@en ."
        ));
    }

    #[test]
    fn test_borrowed_iri_node() {
        let mut store = TripleStore::new();

        let my_namespace = Namespace::new(
            "prefix", "http://www.example.com/"
        ).unwrap();

        let my_reusable_iri = IriNode::new(my_namespace, "ADS::CVMA");

        let quad1 = Triple::new_with_graph(
            // Uses clone interally, but literally just clones the pointers.
            &my_reusable_iri,
            IriNode::new(ARIADNEPLUS, "StainedGlassClass"),
            IriNode::new(RDFS, String::from("label")),
            LangStringLiteral::new_en("Is a piece of stained glass")
        );

        store.add_triple(quad1);

        let quad2 = Triple::new_with_graph(
            &my_reusable_iri,
            IriNode::new(ARIADNEPLUS, "StainedGlassClass"),
            RDF_TYPE,
            IriNode::new(AOCAT, "AO_Resource")
        );

        store.add_triple(quad2);

        let mut buf = vec![];

        store.write_trig(&mut buf).unwrap();

        let string_output = String::from_utf8(buf).unwrap();

        // No guarantee of the output order here, hence .contains().
        assert!(string_output.contains(
            "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n"
        ));
        assert!(string_output.contains(
            "<http://www.example.com/ADS::CVMA> {"
        ));
        assert!(string_output.contains(
            "ariadneplus:StainedGlassClass rdfs:label \"Is a piece of stained glass\"@en ."
        ));
        assert!(string_output.contains(
            "ariadneplus:StainedGlassClass rdf:type aocat:AO_Resource ."
        ));
    }
}