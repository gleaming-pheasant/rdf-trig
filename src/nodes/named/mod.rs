pub mod statics;

use std::borrow::Cow;
use std::io::{self, Write};

use crate::errors::RdfTrigError;
use crate::nodes::{Graph, Node, Object, Predicate, Subject};
use crate::traits::{ToStatic, WriteNQuads};

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

impl WriteNQuads for NamedNode<'_> {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"<")?;
        writer.write_all(self.0.as_bytes())?;
        writer.write_all(b">")?;
        Ok(())
    }
}

impl<'a> Into<Subject<'a>> for NamedNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Named(self)
    }
}

impl<'a> Into<Subject<'a>> for &'a NamedNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Named(self.clone())
    }
}

impl<'a> Into<Predicate<'a>> for NamedNode<'a> {
    #[inline]
    fn into(self) -> Predicate<'a> {
        Predicate(self)
    }
}

impl<'a> Into<Predicate<'a>> for &'a NamedNode<'a> {
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

impl<'a> Into<Object<'a>> for &'a NamedNode<'a> {
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

impl<'a> Into<Graph<'a>> for &'a NamedNode<'a> {
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