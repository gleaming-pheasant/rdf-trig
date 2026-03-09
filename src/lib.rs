//! # rdf-trig
//! A crate for quick formatting of RDF triples in 
//! [TriG](https://en.wikipedia.org/wiki/TriG_(syntax)) from Rust native types.
//! 
//! The main entrypoints for most uses of this crate will be [`DataStore`] and 
//! [`IntoTriple`] of [`IntoTriples`].
//! 
//! Convert types to [`Triple`]s, optionally create [`Graph`]s and add them to 
//! the `DataStore`, either combined or indvidually, and then call 
//! [`WriteTriG::write_trig`] to output the full datastore in *TriG* format to 
//! an implementor of [`io::Write`](std::io::Write) of your choice.
//! 
//! There are lower-level nodes ([Subject](nodes::Subject), 
//! [Predicate](nodes::Predicate) and [Object](nodes::Object)) useful for 
//! converting your types to `Triple`s.
//! 
//! ## Examples
//! ### Convert `IntoTriples`
//! ```rust
//! use rdf_trig::{DataStore, IntoTriple, Triple, WriteTriG};
//! use rdf_trig::namespaces::statics::{AOCAT, ARIADNEPLUS};
//! use rdf_trig::nodes::{Object, Predicate, Subject};
//! 
//! struct MyTriple {
//!     id: usize,
//!     value: &'static str
//! }
//! 
//! impl MyTriple {
//!     fn new(id: usize, value: &'static str) -> MyTriple {
//!         MyTriple { id, value }
//!     }
//! }
//! 
//! impl IntoTriple for MyTriple {
//!     fn into_triple(self) -> Triple {
//!         Triple::new(
//!             Subject::iri(ARIADNEPLUS, self.id.to_string()),
//!             Predicate::new(AOCAT, "has_property"),
//!             Object::string_en(self.value)
//!         )
//!     }
//! }
//! 
//! let mut ds = DataStore::new();
//! ds.add_triple(MyTriple {id: 420, value: "It smells"});
//! 
//! let mut buf = Vec::new();
//! ds.write_trig(&mut buf).unwrap();
//! 
//! let as_string = String::from_utf8(buf).unwrap();
//! assert!(as_string.contains(
//!     "ariadneplus:420 aocat:has_property \"It smells\"@en"
//! ));
//! ```
//! 
//! ### Write Raw `Triple`s
//! ```rust
//! use rdf_trig::{DataStore, Triple, WriteTriG};
//! use rdf_trig::namespaces::statics::{AOCAT, ARIADNEPLUS};
//! use rdf_trig::nodes::{Object, Predicate, Subject};
//! 
//! 
//! let triple = Triple::new(
//!     Subject::iri(ARIADNEPLUS, "my_object"),
//!     Predicate::new(AOCAT, "has_property"),
//!     Object::string_no_lang("is_an_object")
//! );
//! 
//! let mut ds = DataStore::new();
//! ds.add_triple(triple);
//! 
//! let mut buf = Vec::new();
//! ds.write_trig(&mut buf).unwrap();
//! 
//! let as_string = String::from_utf8(buf).unwrap();
//! assert!(as_string.contains(
//!     "ariadneplus:my_object aocat:has_property \"is_an_object\""
//! ));
//! 
//! ```
//! ## A Note on Accepted Values
//! ### *All Escaped Values*
//! For the purposes of speed (and given this crate's limited functionality), 
//! no types are rejected on input. A local name can be passed to this crate 
//! with whitespace, or characters in need of escaping, etc.
//! 
//! Rather than rejecting these inputs and creating a bottleneck, the crate 
//! simply escapes - and in some cases refuses to print - characters on writing 
//! the TriG output. For instance, a local name (such as a prefix) declared with 
//! a line break (\r\n or \n), will be accepted, but the line break will simply 
//! be removed on the output.
//! 
//! __The escape sequences are also not completely valid!__ The crate doesn't 
//! exclude some of the random characters that the TriG specification excludes, 
//! such as this weird exclusion of the multiplication sign:
//! 
//!  > [#0370-#037D] | [#037F-#1FFF]
//! 
//! Instead, the crate hopes to not encounter them, trusts that users will 
//! exclude them manually, or that users graph databases will tolerate the 
//! invalid characters.
//! 
//! Again, this is to improve speed; parsing single bytes rather than verifying 
//! unicode characters.
//! 
//! ### *IRIs*
//! This crate - as with most implementations of RDF - has an awkward 
//! relationship with IRIs.
//! 
//! To a certain degree, it has to trust that param separators, path separators, 
//! etc. are where they should. Using an example from the 
//! [TriG specification](https://www.w3.org/TR/trig/#sec-escapes) to explain: 
//! 
//! > %-encoded sequences are in the character range for IRIs and are explicitly 
//! > allowed in local names. These appear as a '%' followed by two hex 
//! > characters and represent that same sequence of three characters. These 
//! > sequences are not decoded during processing. A term written as 
//! > <http://a.example/%66oo-bar> in TriG designates the IRI 
//! > http://a.example/%66oo-bar and not IRI http://a.example/foo-bar. A term 
//! > written as ex:%66oo-bar with a prefix @prefix ex: <http://a.example/> also 
//! > designates the IRI http://a.example/%66oo-bar.
//! 
//! Therefore, the only verification that this crate does is on namespace IRIs 
//! (the base IRI, and not any endpoints). Once those are verified, any appended 
//! endpoints are simply trusted to be in a valid format. This crate makes no 
//! assumptions about what needs escaping, and instead only escapes characters 
//! that would otherwise make an endpoint completely invalid (such as spaces, 
//! '<', '{', '|', etc.).
//! 
//! ## __Warning__
//! When stored in a [`DataStore`], this crate interns every element that makes 
//! up its structure; [`Graph`]s, [`Triple`]s, [`Quad`]s and [`nodes`]. It 
//! maintains a group of hash-implementing collections and everything is 
//! represented with an index to prevent duplication. This index is converted to 
//! a [`u32`] to be more cache friendly, but be warned, any collection which 
//! reaches over [`u32::MAX`] elements will cause applications to panic.

/* NOTE: This crate uses `.unwrap()` quite a lot when querying interned data. It 
will query an IndexSet (or FastIndexSet), with an objects `...Id(u32)` type. 
Unwrap is called because - except within this crate directly - it is currently 
impossible for these `...Id` types to be created without a corresponding index 
in a `DataStore` field.

Exercise extreme caution if ever developing means that could make these `...Id`s 
constructable by any other method. */
pub mod errors;
mod graphs;
mod groups;
pub mod namespaces;
pub mod nodes;
mod store;
pub(crate) mod traits;
pub(crate) mod utils;

pub use graphs::Graph;
pub use groups::{Quad, Triple};
pub use namespaces::Namespace;
pub use store::DataStore;
pub use traits::{IntoTriple, IntoTriples, WriteTriG};

use std::hash::BuildHasherDefault;
use ahash::AHasher;
/// An extension of `indexmap::IndexSet` which implements the `ahash::AHasher` 
/// hashing algorithm, for quick (though unsecure and undistributable) storage 
/// of interned terms.
pub(crate) type FastIndexSet<T> = indexmap::IndexSet<T, BuildHasherDefault<AHasher>>;

#[cfg(test)]
mod tests {
    use crate::namespaces::statics::{AOCAT, ARIADNEPLUS, FOAF, OWL, RDF};
    use crate::nodes::{Object, Predicate, Subject};
    use crate::traits::WriteTriG;

    use super::*;

    struct MyTriple {
        id: usize,
        value: &'static str
    }

    impl MyTriple {
        fn new(id: usize, value: &'static str) -> MyTriple {
            MyTriple { id, value }
        }
    }

    impl IntoTriple for MyTriple {
        fn into_triple(self) -> Triple {
            Triple::new(
                Subject::iri(ARIADNEPLUS, self.id.to_string()),
                Predicate::new(AOCAT, "has_property"),
                Object::string_en(self.value)
            )
        }
    }

    #[test]
    fn test_into_triple() {
        let mut ds = DataStore::new();
        ds.add_triple(MyTriple {id: 420, value: "It smells"});

        let mut buf = Vec::new();

        ds.write_trig(&mut buf).unwrap();

        let as_string = String::from_utf8(buf).unwrap();

        assert!(as_string.contains(
            "ariadneplus:420 aocat:has_property \"It smells\"@en"
        ));
    }

    #[test]
    fn test_add_triple_to_quad() {
        let mut ds = DataStore::new();
        // Can be updated to be a wrapper around an IriNode! 🤦‍♂️
        let my_graph = Graph::new(ARIADNEPLUS, "MyGraph");
        let graph_id = ds.add_graph(my_graph);

        let triple = MyTriple::new(69, "Is inappropriate");

        ds.add_triple_to_graph(graph_id, triple);

        let mut buf: Vec<u8> = Vec::new();
        ds.write_trig(&mut buf).unwrap();

        let as_str = String::from_utf8(buf).unwrap();

        assert!(as_str.contains("@prefix ariadneplus: <"));
        assert!(as_str.contains("ariadneplus:MyGraph {"));
        assert!(as_str.contains("ariadneplus:69 aocat:has_property"));
        assert!(as_str.contains("\"Is inappropriate\"@en"));
    }

    #[test]
    fn test_add_triple_to_quad_with_escape_chars() {
        let mut ds = DataStore::new();
        
        let my_graph = Graph::new(ARIADNEPLUS, "My\tEscaped Graph");
        let graph_id = ds.add_graph(my_graph);

        let triple = Triple::new(
            Subject::iri(OWL, "Owl\nEscaped Class"),
            Predicate::new(RDF, "has._type"),
            Object::string_no_lang("awkward\r\nliteral")
        );

        ds.add_triple_to_graph(graph_id, triple);

        let mut buf: Vec<u8> = Vec::new();
        ds.write_trig(&mut buf).unwrap();

        let as_str = String::from_utf8(buf).unwrap();

        assert!(as_str.contains("@prefix owl: <http://www.w3.org/2002/07/owl#> ."));
        assert!(as_str.contains("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> ."));
        assert!(as_str.contains(r"ariadneplus:My%09Escaped%20Graph {"));
        assert!(as_str.contains(r"owl:Owl%0AEscaped%20Class rdf:has._type"));
        assert!(as_str.contains(r"awkward\r\nliteral"));
    }

    #[test]
    fn test_add_triple_with_non_string_literal() {
        let mut ds = DataStore::new();
        
        let my_graph = Graph::new(ARIADNEPLUS, "My\rEscaped Graph");
        let graph_id = ds.add_graph(my_graph);

        let triple = Triple::new(
            Subject::iri(OWL, "Class"),
            Predicate::new(RDF, "has type"),
            Object::gyear_from_i32(-420)
        );
        
        ds.add_triple_to_graph(graph_id, triple);

        let mut buf: Vec<u8> = Vec::new();
        ds.write_trig(&mut buf).unwrap();

        let as_str = String::from_utf8(buf).unwrap();

        assert!(as_str.contains("@prefix owl: <http://www.w3.org/2002/07/owl#> ."));
        assert!(as_str.contains("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> ."));
        assert!(as_str.contains("ariadneplus:My%0DEscaped%20Graph {"));
        assert!(as_str.contains("owl:Class rdf:has%20type \"-0420\"^^xsd:gYear"));
    }

    #[test]
    fn test_escaped_prefix() {
        let mut ds = DataStore::new();

        let triple = Triple::new(
            Subject::iri_with_new_namespace(
                "odd~ prefix", 
                "https://www.w3.org/TR/rdf12-schema/#",
                "my_endpoint"
            ).unwrap(),
            Predicate::new(FOAF, "is_friends_with"),
            Object::blank("blank_id")
        );

        ds.add_triple(triple);

        let mut buf = vec![];
        ds.write_trig(&mut buf).unwrap();

        let as_str = String::from_utf8(buf).unwrap();

        println!("{as_str}");

        assert!(as_str.contains(
            "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n"
        ));
        assert!(as_str.contains(
            "@prefix odd\\~prefix: <https://www.w3.org/TR/rdf12-schema/#> .\n"
        ));
        assert!(as_str.contains(
            r"odd\~prefix:my_endpoint foaf:is_friends_with _:blank_id ."
        ));
    }
}