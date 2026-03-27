/// A `BlankNode` is a standard RDF blank node. It serves as a a place to store 
/// known facts about a resource within a graph, without knowing the resource's 
/// specific IRI.
use std::borrow::Cow;
use std::io::{self, Write};

use crate::nodes::Node;
use crate::nodes::subject::Subject;
use crate::nodes::object::Object;
use crate::traits::{ToStatic, WriteNQuads};
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

impl<'a> Into<Object<'a>> for BlankNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Blank(self)
    }
}

impl<'a> Into<Subject<'a>> for BlankNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Blank(self)
    }
}

impl<'a> Into<Object<'a>> for &'a BlankNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Blank(self.clone())
    }
}

impl<'a> Into<Subject<'a>> for &'a BlankNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Blank(self.clone())
    }
}

impl<'a> Into<Node<'a>> for BlankNode<'a> {
    fn into(self) -> Node<'a> {
        Node::Blank(self)
    }
}

impl<'a> ToStatic for BlankNode<'a> {
    type StaticType = BlankNode<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        BlankNode(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl WriteNQuads for BlankNode<'_> {
    #[inline]
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
       writer.write_all(b"_:")?;
        write_escaped_local_name(writer, &self.0)?;

        Ok(()) 
    }
}