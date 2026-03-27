mod index;

use std::collections::hash_map::Iter;
use std::io::{self, Write};

use crate::nodes::{NodeId, NodeStore, Node};
use crate::triples::{
    InternedTriple, InternedTripleId, InternedTripleStore, Triple, TripleView
};
use crate::traits::WriteNQuads;
use crate::triplestore::index::GraphIndex;

/// A `TripleStore` should be the main entry point for applications using this 
/// crate.
/// 
/// Create a `TripleStore`, and use it to register `Triple`s composed of 
/// `Graph`s, `Subject`s, `Predicate`s and `Object`s. These, themselves, being 
/// composed of `BlankNode`s, `LiteralNode`s and `NamedNode`s.
/// 
/// Once your `TripleStore` is built, use it to output your graphs to RDF 
/// formats such as TriG and N-Quads using the relevant traits.
#[derive(Debug)]
pub struct TripleStore {
    nodes: NodeStore,
    triples: InternedTripleStore,
    graph_index: GraphIndex
}

impl TripleStore {
    /// Create a new `TripleStore`.
    pub fn new() -> TripleStore {
        TripleStore {
            nodes: NodeStore::new(),
            triples: InternedTripleStore::new(),
            graph_index: GraphIndex::new()
        }
    }

    /// Add a `Triple` (or impl `Into<Triple>`) to this `TripleStore`.
    pub fn add_triple<'a, T: Into<Triple<'a>>>(&mut self, triple: T) {
        let triple = self.intern_triple(triple.into());
        
        // Get a clone of the `NodeId` for the graph for adding to the index.
        let graph_id = triple.graph();

        let interned_triple_id = self.triples.intern_triple(triple);

        // Add to index.
        self.graph_index.add_triple(graph_id, interned_triple_id);
    }

    /// Add all of the `Node`s in the provided `Triple` to this `TripleStore`'s 
    /// `NodeStore`, returning an `InternedTriple` wrapper around the `NodeId` 
    /// for each `Node`.
    fn intern_triple(
        &mut self, triple: Triple<'_>
    ) -> InternedTriple {
        let (graph, subject, predicate, object) = triple.into_parts();

        let graph = graph.map(|sg| self.intern_node(sg.into()));
        let subject = self.intern_node(subject.into());
        let predicate = self.intern_node(predicate.into());
        let object = self.intern_node(object.into());

        InternedTriple::new(
            graph, subject, predicate, object
        )
    }

    /// Add a `Node` to the `NodeStore` of this `TripleStore`.
    fn intern_node(&mut self, staging_node: Node<'_>) -> NodeId {
        self.nodes.intern_node(staging_node)
    }

    /// Retrieve a `Node` reference for the given `NodeId`.
    fn resolve_node(&self, node_id: NodeId) -> &Node<'static> {
        self.nodes.query_node(node_id)
    }

    /// Retrieve a `Triple` for the given `InternedTripleId`.
    /// 
    /// This function should be used for RDF output formats such as TriG, where 
    /// the graph index is useful for grouping the output.
    fn resolve_interned_triple_from_id(
        &self, triple_id: InternedTripleId
    ) -> TripleView<'_> {
        let interned_triple = self.triples.query_triple(triple_id);
        self.resolve_interned_triple(interned_triple)
    }

    /// Take an `InternedTriple` and turn it into a `TripleView` by resolving 
    /// all of its contained `Node`s.
    fn resolve_interned_triple(
        &self, interned_triple: &InternedTriple
    ) -> TripleView<'_> {
        TripleView::new(
            self.resolve_node(*interned_triple.subject()),
            self.resolve_node(*interned_triple.predicate()),
            self.resolve_node(*interned_triple.object()),
            interned_triple.graph()
                .and_then(|gn_id| Some(self.resolve_node(gn_id)))
        )
    }

    /// Retrieve an iterator of all graphs and their corresponding 
    /// `InternedTripleId`s in the `graph_index` of this `TripleStore`.
    fn graphs_iter(&self)
    -> Iter<'_, Option<NodeId>, Vec<InternedTripleId>> {
        self.graph_index.iter()
    }

    /// Retrieve all interned `Triple`s as an iterator over `TripleView`s.
    /// 
    /// Each stored `InternedTriple` is collected from the `triples` 
    /// `InternedTripleStore` and resolved to `Node`s.
    fn triples_iter(&self) -> impl Iterator<Item = TripleView<'_>> {
        self.triples.iter()
            .map(|it| self.resolve_interned_triple(it))
    }
}

impl WriteNQuads for TripleStore {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // With N-Quads, we don't care about graph order. Would use the index if 
        // we were still writing in TriG.
        for triple in self.triples_iter() {
            triple.write_nquads(writer)?;
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::namespaces::statics::{AOCAT, ARIADNEPLUS, RDFS};
//     use crate::nodes::predicate::{RDF_TYPE};
//     use crate::nodes::{IriNode, LangStringLiteral};

//     use super::*;

//     #[test]
//     fn test_expected_triple_write_trig() {
//         let mut store = TripleStore::new();

//         let triple = Triple::new(
//             IriNode::new(ARIADNEPLUS, "My::Class/123"),
//             IriNode::new(RDFS, String::from("label")),
//             LangStringLiteral::new_en("Is a\tspecial class")
//         );

//         store.add_triple(triple);

//         let mut buf = vec![];

//         store.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from(
//                 "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n@prefix ariadneplus: <https://ariadne-infrastructure.eu/aocat/> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n<https://ariadne-infrastructure.eu/aocat/My::Class/123> rdfs:label \"Is a\\tspecial class\"@en .\n\n"
//             )
//         );
//     }

//     #[test]
//     fn test_expected_triple_and_graph_write_trig() {
//         let mut store = TripleStore::new();

//         let my_namespace = Namespace::new(
//             "prefix", "http://www.example.com/"
//         ).unwrap();

//         let quad = Triple::new_with_graph(
//             IriNode::new(my_namespace, "SND::Archaeology"),
//             IriNode::new(ARIADNEPLUS, "StainedGlassClass"),
//             IriNode::new(RDFS, String::from("label")),
//             LangStringLiteral::new_en("Is a piece of stained glass")
//         );

//         store.add_triple(quad);

//         let triple = Triple::new(
//             IriNode::new(ARIADNEPLUS, "Natural History Museum"),
//             RDF_TYPE,
//             IriNode::new(AOCAT, "AO_Agent")
//         );

//         store.add_triple(triple);

//         let mut buf = vec![];

//         store.write_trig(&mut buf).unwrap();

//         let string_output = String::from_utf8(buf).unwrap();

//         // No guarantee of the output order here, hence .contains().
//         assert!(string_output.contains(
//             "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n"
//         ));
//         assert!(string_output.contains(
//             "<http://www.example.com/SND::Archaeology> {"
//         ));
//         assert!(string_output.contains(
//             "{ ariadneplus:StainedGlassClass rdfs:label \"Is a piece of stained glass\"@en ."
//         ));
//     }

//     #[test]
//     fn test_borrowed_iri_node() {
//         let mut store = TripleStore::new();

//         let my_namespace = Namespace::new(
//             "prefix", "http://www.example.com/"
//         ).unwrap();

//         let my_reusable_iri = IriNode::new(my_namespace, "ADS::CVMA");

//         let quad1 = Triple::new_with_graph(
//             // Uses clone interally, but literally just clones the pointers.
//             &my_reusable_iri,
//             IriNode::new(ARIADNEPLUS, "StainedGlassClass"),
//             IriNode::new(RDFS, String::from("label")),
//             LangStringLiteral::new_en("Is a piece of stained glass")
//         );

//         store.add_triple(quad1);

//         let quad2 = Triple::new_with_graph(
//             &my_reusable_iri,
//             IriNode::new(ARIADNEPLUS, "StainedGlassClass"),
//             RDF_TYPE,
//             IriNode::new(AOCAT, "AO_Resource")
//         );

//         store.add_triple(quad2);

//         let mut buf = vec![];

//         store.write_trig(&mut buf).unwrap();

//         let string_output = String::from_utf8(buf).unwrap();

//         // No guarantee of the output order here, hence .contains().
//         assert!(string_output.contains(
//             "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n"
//         ));
//         assert!(string_output.contains(
//             "<http://www.example.com/ADS::CVMA> {"
//         ));
//         assert!(string_output.contains(
//             "ariadneplus:StainedGlassClass rdfs:label \"Is a piece of stained glass\"@en ."
//         ));
//         assert!(string_output.contains(
//             "ariadneplus:StainedGlassClass a aocat:AO_Resource ."
//         ));
//     }
// }