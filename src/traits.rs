use std::io::{Result as IoResult, Write};

use crate::triples::Triple;

/// An implementation of [`Write`] which writes in the 
/// [TriG](https://en.wikipedia.org/wiki/TriG_(syntax)) format.
pub trait WriteTriG {
    /// Write self to the provided writer in TriG format.
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()>;
}

/// A blanket implementation of [`WriteTriG`], which allows references to 
/// implementors to automatically implement the trait.
/// 
/// The main trait does not currently accept generics for implementors, so 
/// references will work by derived derefs, but this implementation will future-
/// proof this in case of changes.
impl<'a, T: WriteTriG + ?Sized> WriteTriG for &'a T {
    #[inline]
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        (**self).write_trig(writer)
    }
}

/// A trait for converting self into a single [`Triple`]s.
pub trait IntoTriple {
    /// Convert `self` into a `Triple`.
    fn into_triple<'a>(self) -> Triple<'a>;
}

pub(crate) trait ToInterned {
    type InternedType;

    /// Implementors of this trait must take a reference to self and return an 
    /// `InternedType` for self which is self with a 'static lifetime.
    fn to_interned(&self) -> Self::InternedType;
}