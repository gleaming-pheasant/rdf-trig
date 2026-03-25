use std::io::{Result as IoResult, Write};

/// An implementation of [`Write`] which writes in the 
/// [N-Quads](https://www.w3.org/TR/n-quads/) format.
pub trait WriteNQuads {
    /// Write self to the provided writer in N-Quads format.
    fn write_nquads<W: Write>(&self, writer: &mut W) -> IoResult<()>;
}

/// A blanket implementation of [`WriteNQuads`], which allows references to 
/// implementors to automatically implement the trait.
/// 
/// The main trait does not currently accept generics for implementors, so 
/// references will work by derived derefs, but this implementation will future-
/// proof this in case of changes.
impl<'a, T: WriteNQuads + ?Sized> WriteNQuads for &'a T {
    #[inline]
    fn write_nquads<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        (**self).write_nquads(writer)
    }
}

pub(crate) trait ToStatic {
    type StaticType;

    /// Implementors of this trait must take a reference to self and return an 
    /// `StaticType` for self which is self with a 'static lifetime.
    fn to_static(&self) -> Self::StaticType;
}