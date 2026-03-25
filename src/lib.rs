#![doc = include_str!("../README.md")]
/* NOTE: This crate uses `.unwrap()` quite a lot when querying interned data. It 
will query an IndexSet (or FastIndexSet), with an objects `...Id(u32)` type. 
Unwrap is called because - except within this crate directly - it is currently 
impossible for these `...Id` types to be created without a corresponding index 
in a `TripleStore` field.

Exercise extreme caution if ever developing means that could make these `...Id`s 
constructable by any other method. */
pub mod errors;
pub mod nodes;
pub(crate) mod traits;
mod triples;
mod triplestore;
pub(crate) mod utils;

pub use triplestore::TripleStore;
pub use nodes::{
    BlankNode, BooleanLiteral, DateTimeLiteral, DecimalLiteral, GYearLiteral, 
    IriNode, LangStringLiteral, LiteralNode
};
pub use triples::Triple;
pub use traits::WriteNQuads;

use std::hash::BuildHasherDefault;
use ahash::AHasher;
/// An extension of `indexmap::IndexSet` which implements the `ahash::AHasher` 
/// hashing algorithm, for quick (though unsecure and undistributable) storage 
/// of interned terms.
pub(crate) type FastIndexSet<T> = indexmap::IndexSet<T, BuildHasherDefault<AHasher>>;