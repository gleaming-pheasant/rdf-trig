/// A `BlankNode` is a standard RDF blank node. It serves as a a place to store 
/// known facts about a resource within a graph, without knowing the resource's 
/// specific IRI.
/// 
/// `BlankNode` directly implements [`WriteTriG`], prefixing the provided id 
/// with the standard blank node "_:" prefix.
/// 
/// `BlankNode`s cannot be initialised directly, and must be generated as part 
/// of [`Subject`] or [`Object`] constructors.
use std::borrow::Cow;
use std::io::{self, Write};

use crate::nodes::subject::Subject;
use crate::nodes::object::Object;
use crate::nodes::store::{InternedBlankNode, StagedNode};
use crate::traits::WriteTriG;
use crate::utils::write_escaped_local_name;

/// A `BlankNode` is simply a node with a `str` label. This crate relies on the 
/// caller to manage any corresponding `Predicate`s and `Object`s.
/// 
/// No character escaping is done on the label before or during construction.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BlankNode<'a>(pub(crate) Cow<'a, str>);

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
    fn into(self) -> Object<'a> {
        Object::Blank(self)
    }
}

impl<'a> Into<Subject<'a>> for BlankNode<'a> {
    fn into(self) -> Subject<'a> {
        Subject::Blank(self)
    }
}

impl<'a> Into<StagedNode<'a>> for BlankNode<'a> {
    /// Wrap this `BlankNode` as a `StagedNode` in preparation for interning.
    #[inline]
    fn into(self) -> StagedNode<'a> {
        StagedNode::Blank(self)
    }
}

impl ToOwned for BlankNode<'_> {
    type Owned = InternedBlankNode;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        InternedBlankNode(BlankNode(Cow::Owned(self.0.clone().into_owned())))
    }
}

impl WriteTriG for BlankNode<'_> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"_:")?;
        write_escaped_local_name(writer, &self.0)?;

        Ok(())
    }
}