#![doc = include_str!("../README.md")]
/* NOTE: This crate uses `.unwrap()` when querying interned data. It will query 
IndexSets, with a `NodeId` or `TripleId`. Unwrap is called because it is 
currently impossible for these `Id` types to be created externally without a 
guarantee that the `Id` corresponds to an interned item.

Exercise extreme caution if ever developing means that could make these `Id`s 
constructable by any other method, or removable after being interned. */
pub mod errors;
pub mod nodes;
pub(crate) mod traits;
mod triples;
mod triplestore;
pub(crate) mod utils;

pub use triplestore::TripleStore;
pub use nodes::{
    BlankNode, BooleanLiteral, DateTimeLiteral, DecimalLiteral, GYearLiteral, 
    NamedNode, LiteralNode, StringLiteral, named::statics
};
pub use triples::Triple;
pub use traits::WriteNQuads;

use std::hash::BuildHasherDefault;
use ahash::AHasher;
/// An extension of `indexmap::IndexSet` which implements the `ahash::AHasher` 
/// hashing algorithm, for quick (though unsecure and undistributable) storage 
/// of interned terms.
pub(crate) type FastIndexSet<T> = indexmap::IndexSet<T, BuildHasherDefault<AHasher>>;