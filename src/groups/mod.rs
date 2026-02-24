//! This module contains code for [`Triple`]s and [`Quad`]s; two closely 
//! related, but distinct, groupings of objects in RDF.
//! 
//! A `Triple` is a classic `subject`, `predicate` and `object`. A `Quad` 
//! meanwhile assigns a `Triple` to a `Graph` (through a registered `Graph`'s 
//! `GraphId` in the case of this crate).
pub(crate) mod quads;
pub(crate) mod triples;

pub use quads::Quad;
pub use triples::Triple;