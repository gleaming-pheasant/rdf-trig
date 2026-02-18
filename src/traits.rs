use std::io::{self, Write};

use crate::groups::triples::Triple;

/// An implementation of [`Write`] which writes in the 
/// [TriG](https://en.wikipedia.org/wiki/TriG_(syntax)) format.
pub(crate) trait WriteTriG {
    /// Write self to the provided writer in TriG format.
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

/// A trait for converting self into a single [`Triple`]s.
pub trait IntoTriple {
    /// Convert `self` into a `Triple`.
    fn into_triple(self) -> Triple;
}

/// A trait for converting self to an iterator over [`Triple`]s.
pub trait IntoTriples {
    type Iter: Iterator<Item = Triple>;

    /// Convert `self` into a [`Iterator<Item = Triple>`].
    fn into_triples(self) -> Self::Iter;
}