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
mod graphs;
mod groups;
mod namespaces;
pub mod nodes;
mod store;
pub(crate) mod traits;

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

    #[test]
    fn test_into_triple() {
        struct MyTriple {
            id: usize,
            value: &'static str
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

        let mut ds = DataStore::new();
        ds.add_triple(MyTriple {id: 420, value: "It smells"});

        let mut buf = Vec::new();

        ds.write_trig(&mut buf).unwrap();

        let as_string = String::from_utf8(buf).unwrap();

        println!("{as_string}");

        assert!(as_string.contains(
            "ariadneplus:420 aocat:has_property \"It smells\"@en"
        ));
    }

    #[test]
    fn quick_mafs() {
        let two_plus_two = 2 + 2;
        assert_eq!(two_plus_two, 4);

        let minus_one = two_plus_two - 1;
        assert_eq!(minus_one, 3);
    }
}