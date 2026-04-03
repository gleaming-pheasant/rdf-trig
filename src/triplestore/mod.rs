mod trig_impl;
use trig_impl::TriGStore;

use std::io::{self, Write};

use crate::nodes::{NodeId, NodeStore, Node};
use crate::triples::{
    InternedTriple,
    InternedTripleStore,
    Triple,
    TripleView
};
use crate::traits::{WriteNQuads, WriteTriG};
#[cfg(feature = "tokio")]
use crate::traits::{WriteNQuadsAsync, WriteTriGAsync};

/// A `TripleStore` should be the main entry point for applications using this 
/// crate.
/// 
/// Create a `TripleStore`, and use it to register `Triple`s composed of 
/// `Graph`s, `Subject`s, `Predicate`s and `Object`s. These, themselves, being 
/// composed of `BlankNode`s, `LiteralNode`s and `NamedNode`s.
/// 
/// Once your `TripleStore` is built, use it to output your graphs to RDF 
/// formats `TriG` and `N-Quads` using the relevant traits.
#[derive(Debug)]
pub struct TripleStore {
    nodes: NodeStore,
    triples: InternedTripleStore
}

impl TripleStore {
    /// Create a new `TripleStore`.
    pub fn new() -> TripleStore {
        TripleStore {
            nodes: NodeStore::new(),
            triples: InternedTripleStore::new()
        }
    }

    /// Add a `Triple` (or impl `Into<Triple>`) to this `TripleStore`.
    pub fn add_triple<'a, T: Into<Triple<'a>>>(&mut self, triple: T) {
        let interned_triple = self.intern_triple(triple.into());
        self.triples.intern_triple(interned_triple);
    }

    /// Add all of the `Node`s in the provided `Triple` to this `TripleStore`'s 
    /// `NodeStore`, returning an `InternedTriple` wrapper around the `NodeId` 
    /// for each `Node`.
    fn intern_triple<'a>(
        &mut self, triple: Triple<'a>
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
    fn intern_node<'a>(&mut self, staging_node: Node<'a>) -> NodeId {
        self.nodes.intern_node(staging_node)
    }

    /// Retrieve a `Node` reference for the given `NodeId`.
    fn resolve_node(&self, node_id: NodeId) -> &Node<'static> {
        self.nodes.query_node(node_id)
    }

    /// Take an `InternedTriple` and turn it into a `TripleView` by resolving 
    /// all of its contained `Node`s.
    fn resolve_triple_view_from_interned<'a>(
        &'a self, interned_triple: &InternedTriple
    ) -> TripleView<'a> {
        TripleView::new(
            interned_triple.graph()
                .and_then(|gn_id| Some(self.resolve_node(gn_id))),
            self.resolve_node(interned_triple.subject()),
            self.resolve_node(interned_triple.predicate()),
            self.resolve_node(interned_triple.object())
        )
    }

    /// Retrieve a `Vec` of `NodeId`s for all `InternedTriple`s contained in 
    /// this `TripleStore`, which can the be sorted to retrieve a Compressed 
    /// Sparse Row matrix, ordered for outputting values via `TriG`
    /// 
    /// This is a costly O(*n* log *n*) operation, and should only be used when 
    /// outputting to TriG; on the grounds that the overhead of sorting the data 
    /// is acceptable for the savings in written output size.
    fn to_trig_store(&self) -> TriGStore {
        // Costly, but must be a Vec to implement sort.
        TriGStore::new(self.triples.iter().collect())
    }

    /// Retrieve all interned `Triple`s as an iterator over `TripleView`s.
    /// 
    /// Each stored `InternedTriple` is collected from the `triples` 
    /// `InternedTripleStore` and resolved to `Node`s.
    fn triples_iter<'a>(&'a self) -> impl Iterator<Item = TripleView<'a>> {
        self.triples.iter()
            .map(|it| self.resolve_triple_view_from_interned(it))
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

impl WriteTriG for TripleStore {
    /// Calls a specialised version - write_store_trig() - to separate out the 
    /// Compressed Sparse Row logic.
    #[inline]
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.to_trig_store().write_store_trig(writer, &self.nodes)
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::{BlankNode, DateTimeLiteral, LiteralNode, NamedNode, StringLiteral};
    use crate::nodes::statics::{aocat, owl, rdf, rdfs};

    use super::*;

    #[test]
    fn test_write_nquads() {
        let mut ts = TripleStore::new();

        let owned_graph = "http://www.example.com/MyGraph".to_string();
        let owned_resource1 = "https://example.com/resources/MyResource"
            .to_string();

        ts.add_triple(
            Triple::new_with_graph(
                NamedNode::new(owned_graph.clone()).unwrap().into(),
                NamedNode::new(owned_resource1.clone()).unwrap().into(),
                rdf::Property::Type.into(),
                owl::Class::Thing.into()
            )
        );

        let resource2 = "http://www.example.com/MyOtherResource";

        ts.add_triple(
            Triple::new(
                NamedNode::new(resource2).unwrap().into(),
                rdf::Property::Type.into(),
                aocat::Class::AoIndividualDataResource.into()
            )
        );

        let mut buf = vec![];
        ts.write_nquads(&mut buf).unwrap();
        let nquads_string = String::from_utf8(buf).unwrap();

        assert_eq!(
            nquads_string,
            format!(
                "<{}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Thing> <{}> .\n\
                <{}> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://www.ariadne-infrastructure.eu/resource/ao/cat/1.1/AO_Individual_Data_Resource> .\n",
                owned_resource1, owned_graph, resource2
            )
        );
    }

    #[test]
    fn test_write_nquads_with_literals() {
        let mut ts = TripleStore::new();

        ts.add_triple(Triple::new(
            NamedNode::new("urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36").unwrap().into(),
            aocat::Property::WasCreatedOn.into(),
            DateTimeLiteral::try_from_str("1969-10-12T12:59:30Z").unwrap().into()
        ));

        ts.add_triple(Triple::new(
            BlankNode::new("unknown~node").into(),
            rdfs::Property::Label.into(),
            StringLiteral::new("mon node", Some("fr")).unwrap().into()
        ));

        let mut buf = vec![];
        ts.write_nquads(&mut buf).unwrap();
        let nquads_string = String::from_utf8(buf).unwrap();

        assert_eq!(
            nquads_string,
            "<urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36> <https://www.ariadne-infrastructure.eu/resource/ao/cat/1.1/was_created_on> \"1969-10-12T12:59:30Z\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .\n\
            _:unknown\\~node <http://www.w3.org/2000/01/rdf-schema#label> \"mon node\"@fr .\n"
        );
    }

    #[test]
    fn test_write_trig() {
        // Owned strings are already tested in previous tests.
        let graph = "urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36";
        let graph_subject = "https://www.example.com/MyResource";

        let outer_subject = "https://www.example.com/MyOuterResource";

        let mut ts = TripleStore::new();

        ts.add_triple(Triple::new_with_graph(
            NamedNode::new(graph).unwrap().into(),
            NamedNode::new(graph_subject).unwrap().into(),
            rdf::Property::Type.into(),
            owl::Class::Thing.into()
        ));

        ts.add_triple(Triple::new_with_graph(
            NamedNode::new(graph).unwrap().into(),
            NamedNode::new(graph_subject).unwrap().into(),
            owl::Property::OneOf.into(),
            LiteralNode::new("This Thing").into()
        ));

        ts.add_triple(Triple::new_with_graph(
            NamedNode::new(graph).unwrap().into(),
            NamedNode::new(graph_subject).unwrap().into(),
            owl::Property::OneOf.into(),
            LiteralNode::new("This Other Thing").into()
        ));

        ts.add_triple(Triple::new(
            NamedNode::new(outer_subject).unwrap().into(),
            aocat::Property::HasPart.into(),
            NamedNode::new(graph_subject).unwrap().into()
        ));

        let mut buf = vec![];
        ts.write_trig(&mut buf).unwrap();
        let trig_string = String::from_utf8(buf).unwrap();
        
        assert_eq!(
            trig_string,
            // Default Graph (None) is always first in `Ord`
            "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> . \
            <https://www.example.com/MyOuterResource> <https://www.ariadne-infrastructure.eu/resource/ao/cat/1.1/has_part> <https://www.example.com/MyResource> . \
            <urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36> { \
            <https://www.example.com/MyResource> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Thing> ; \
            <http://www.w3.org/2002/07/owl#oneOf> \"This Thing\" , \"This Other Thing\" . } "
        );
    }
}