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
//! 
//! ## A Note on *IRI*s
//! This crate - as with most implementations of RDF - has an awkward 
//! relationship with IRIs.
//! 
//! To a certain degree, it has to trust that param separators, path separators, 
//! etc. are where they should. Using an example from the 
//! [TriG specification](https://www.w3.org/TR/trig/#sec-escapes) to explain: 
//! > %-encoded sequences are in the character range for IRIs and are explicitly 
//! > allowed in local names. These appear as a '%' followed by two hex 
//! > characters and represent that same sequence of three characters. These 
//! > sequences are not decoded during processing. A term written as 
//! > <http://a.example/%66oo-bar> in TriG designates the IRI 
//! > http://a.example/%66oo-bar and not IRI http://a.example/foo-bar. A term 
//! > written as ex:%66oo-bar with a prefix @prefix ex: <http://a.example/> also 
//! > designates the IRI http://a.example/%66oo-bar.
//! 
//! Therefore, the only percent-encoding that this crate does, is non-printable 
//! ASCII (00-1F and 7F), unsafe characters and non-ASCII characters.
//! 
//! No validation on the layout of the URL is performed at all.
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
    use crate::namespaces::statics::{AOCAT, ARIADNEPLUS};
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
}