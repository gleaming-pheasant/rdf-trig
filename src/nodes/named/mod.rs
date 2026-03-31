pub mod statics;

use std::borrow::Cow;
use std::io::{self, Write};

use crate::errors::RdfTrigError;
use crate::nodes::{Graph, Node, Object, Predicate, Subject};
use crate::traits::{ToStatic, WriteNQuads, WriteTriG};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NamedNode<'a>(Cow<'a, str>);

impl<'a> NamedNode<'a> {
    /// Create a new `NamedNode`. Returns an error if `iri` is not a valid `iri`.
    pub fn new<C: Into<Cow<'a, str>>>(iri: C) -> Result<NamedNode<'a>, RdfTrigError> {
        let iri = iri.into();
        
        // fluent_uri implement zero-copy parsing.
        if fluent_uri::Iri::parse(&*iri).is_err() {
            return Err(RdfTrigError::InvalidIri(iri.to_string()));
        }

        Ok(NamedNode(iri))
    }

    /// A private function for generating static IRIs for widely used resources, 
    /// properties and classes.
    pub(crate) const fn new_const(iri: &'static str) -> NamedNode<'static> {
        NamedNode(Cow::Borrowed(iri))
    }
}

impl<'a> Into<Subject<'a>> for NamedNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Named(self)
    }
}

impl<'a, 'b> Into<Subject<'a>> for &'b NamedNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Named(NamedNode(Cow::Owned(self.0.clone().into_owned())))
    }
}

impl<'a> Into<Predicate<'a>> for NamedNode<'a> {
    #[inline]
    fn into(self) -> Predicate<'a> {
        Predicate(self)
    }
}

impl<'a, 'b> Into<Predicate<'a>> for &'b NamedNode<'a> {
    #[inline]
    fn into(self) -> Predicate<'a> {
        Predicate(self.clone())
    }
}

impl<'a> Into<Object<'a>> for NamedNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Named(self)
    }
}

impl<'a, 'b> Into<Object<'a>> for &'b NamedNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Named(self.clone())
    }
}

impl<'a> Into<Graph<'a>> for NamedNode<'a> {
    #[inline]
    fn into(self) -> Graph<'a> {
        Graph(self)
    }
}

impl<'a, 'b> Into<Graph<'a>> for &'b NamedNode<'a> {
    #[inline]
    fn into(self) -> Graph<'a> {
        Graph(self.clone())
    }
}

impl<'a> Into<Node<'a>> for NamedNode<'a> {
    fn into(self) -> Node<'a> {
        Node::Named(self)
    }
}

impl<'a> ToStatic for NamedNode<'a> {
    type StaticType = NamedNode<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        NamedNode(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> WriteNQuads for NamedNode<'a> {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"<")?;
        writer.write_all(self.0.as_bytes())?;
        writer.write_all(b">")?;
        Ok(())
    }
}

impl<'a> WriteTriG for NamedNode<'a> {
    #[inline]
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // Crate doesn't add prefixes or shorten NamedNodes, so same as N-Quads.
        self.write_nquads(writer)
    }
}