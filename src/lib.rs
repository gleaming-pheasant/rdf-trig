//! __Warning!__ When stored in a [`DataStore`], this crate interns every 
//! element that makes up its structure; [`Graph`]s, [`Triple`]s, [`Quad`]s and 
//! [`nodes`]. It maintains a group of hash-implementing collections and 
//! everything is represented with an index to prevent duplication. This index 
//! is converted to a [`u32`] to be more cache friendly, but be warned, any 
//! collection which reaches over [`u32::MAX`] element will cause applications 
//! to panic.

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
mod namespaces;
pub mod nodes;
mod store;
pub(crate) mod traits;
pub(crate) mod utils;

pub use graphs::Graph;
pub use groups::{Quad, Triple};
pub use namespaces::Namespace;
pub use store::DataStore;
pub use traits::{IntoTriple, IntoTriples};

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