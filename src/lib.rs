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
    #[test]
    fn quick_mafs() {
        let two_plus_two = 2 + 2;
        assert_eq!(two_plus_two, 4);

        let minus_one = two_plus_two - 1;
        assert_eq!(minus_one, 3);
    }
}