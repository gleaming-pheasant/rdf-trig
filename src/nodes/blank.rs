/// A `BlankNode` is a standard RDF blank node. It serves as a a place to store 
/// known facts about a resource within a graph, without knowing the resource's 
/// specific IRI.
use std::borrow::Cow;
use std::io::{self, Write};

use crate::traits::{ToStatic, WriteNQuads, WriteTriG};
#[cfg(feature = "tokio")]
use crate::traits::{WriteNQuadsAsync, WriteTriGAsync};
use crate::utils::write_escaped_local_name;

/// A `BlankNode` is simply a node with a `str` label. This crate relies on the 
/// caller to manage any corresponding `Predicate`s and `Object`s.
/// 
/// No character escaping is done on the label before or during construction.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlankNode<'a>(Cow<'a, str>);

impl<'a> BlankNode<'a> {
    /// Create a new `BlankNode` with the provided `id`.
    pub fn new<C: Into<Cow<'a, str>>>(id: C) -> BlankNode<'a> {
        BlankNode(id.into())
    }

    /// Get a reference to the label for this `BlankNode`.
    pub fn label(&self)  -> &str {
        &self.0
    }
}

impl<'a> ToStatic for BlankNode<'a> {
    type StaticType = BlankNode<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        BlankNode(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> WriteNQuads for BlankNode<'a> {
    #[inline]
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
       writer.write_all(b"_:")?;
        write_escaped_local_name(writer, &self.0)?;

        Ok(()) 
    }
}

impl<'a> WriteTriG for BlankNode<'a> {
    #[inline]
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
       // Identical representation in both TriG and N-Quads
       self.write_nquads(writer)
    }
}