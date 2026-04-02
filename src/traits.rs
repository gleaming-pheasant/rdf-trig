use std::io::{self, Write};

#[cfg(feature = "tokio")]
use tokio::io::AsyncWrite;

/// An implementation of [`Write`] which writes in the 
/// [N-Quads](https://www.w3.org/TR/n-quads/) format.
/// 
/// Prefer implementing and using `WriteTriG` where file/stream size takes 
/// precedence over compute overhead.
/// 
/// Implementors of `WriteNQuads` in this crate write whole `NamedNode`s and 
/// don't care about the presence or order of graphs or other named nodes.
pub trait WriteNQuads {
    /// Write self to the provided writer in N-Quads format.
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

/// A blanket implementation of [`WriteNQuads`], which allows references to 
/// implementors to automatically implement the trait.
/// 
/// The main trait does not currently accept generics for implementors, so 
/// references will work by derived derefs, but this implementation will future-
/// proof this in case of changes.
impl<'a, T: WriteNQuads + ?Sized> WriteNQuads for &'a T {
    #[inline]
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        (**self).write_nquads(writer)
    }
}

/// An implementation of [`Write`] which writes in the 
/// [TriG](https://www.w3.org/TR/trig/) format.
/// 
/// Prefer implementing and using `WriteNQuads` where raw output speed is 
/// prioritised over compute overhead.
/// 
/// Implementors of `WriteTriG` in this crate index `Graph`s and order `Subject` 
/// writing to reduce the number of times `NamedNode`s and `Graph`s are written. 
/// They do not, however, add prefixes to any IRIs (even "rdf:type" or "a") 
/// expect XSD type declarations.
pub trait WriteTriG {
    /// Write self to the provided writer in TriG format.
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

/// A blanket implementation of [`WriteNQuads`], which allows references to 
/// implementors to automatically implement the trait.
/// 
/// The main trait does not currently accept generics for implementors, so 
/// references will work by derived derefs, but this implementation will future-
/// proof this in case of changes.
impl<'a, T: WriteTriG + ?Sized> WriteTriG for &'a T {
    #[inline]
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        (**self).write_trig(writer)
    }
}

pub(crate) trait ToStatic {
    type StaticType;

    /// Implementors of this trait must take a reference to self and return an 
    /// `StaticType` for self which is self with a 'static lifetime.
    fn to_static(&self) -> Self::StaticType;
}

#[cfg(feature = "tokio")]
/// An implementation of [`AsyncWrite`] which writes in the 
/// [N-Quads](https://www.w3.org/TR/n-quads/) format.
/// 
/// Prefer implementing and using `WriteTriGAsync` where file/stream size takes 
/// precedence over compute overhead.
/// 
/// Implementors of `WriteNQuadsAsync` in this crate write whole `NamedNode`s 
/// and don't care about the presence or order of graphs or other named nodes.
pub trait WriteNQuadsAsync {
    /// Write self to the provided writer in N-Quads format.
    fn write_nquads_async<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin + Send;
}

#[cfg(feature = "tokio")]
/// A blanket implementation of [`WriteNQuadsAsync`], which allows references to 
/// implementors to automatically implement the trait.
/// 
/// The main trait does not currently accept generics for implementors, so 
/// references will work by derived derefs, but this implementation will future
/// proof this in case of changes.
impl<'a, T: WriteNQuadsAsync + ?Sized> WriteNQuadsAsync for &'a T {
    #[inline]
    fn write_nquads_async<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin + Send
    {
        (**self).write_nquads_async(writer)
    }
}

#[cfg(feature = "tokio")]
/// An implementation of [`AsyncWrite`] which writes in the 
/// [TriG](https://www.w3.org/TR/trig/) format.
/// 
/// Prefer implementing and using `WriteNQuadsAsync` where raw output speed is 
/// prioritised over compute overhead.
/// 
/// Implementors of `WriteTriG` in this crate index `Graph`s and order `Subject` 
/// writing to reduce the number of times `NamedNode`s and `Graph`s are written. 
/// They do not, however, add prefixes to any IRIs (even "rdf:type" or "a") 
/// expect XSD type declarations.
pub trait WriteTriGAsync {
    /// Write self to the provided writer in TriG format.
    fn write_trig_async<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin + Send;
}

#[cfg(feature = "tokio")]
/// A blanket implementation of [`WriteNQuads`], which allows references to 
/// implementors to automatically implement the trait.
/// 
/// The main trait does not currently accept generics for implementors, so 
/// references will work by derived derefs, but this implementation will future-
/// proof this in case of changes.
impl<'a, T: WriteTriGAsync + ?Sized> WriteTriGAsync for &'a T {
    #[inline]
    fn write_trig_async<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin + Send
    {
        (**self).write_trig_async(writer)
    }
}