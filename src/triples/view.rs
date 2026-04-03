use std::io::{self, Write};

use crate::nodes::Node;
use crate::traits::WriteNQuads;
#[cfg(feature = "tokio")]
use crate::traits::WriteNQuadsAsync;

/// A `TripleView` serves as a means to view a [`Triple`] as raw [`Node`]s. This 
/// allows an interned `Triple` to be formatted in RDF formats (e.g. with 
/// [`WriteNQuads`]), without re-abstracting to [`Subject`]s, [`Predicate`]s or 
/// [`Object`]s.
pub(crate) struct TripleView<'a> {
    graph: Option<&'a Node<'static>>,
    subject: &'a Node<'static>,
    predicate: &'a Node<'static>,
    object: &'a Node<'static>
}

impl<'a> TripleView<'a> {
    /// Create a new `TripleView` from parts.
    pub(crate) fn new(
        graph: Option<&'a Node<'static>>,
        subject: &'a Node<'static>,
        predicate: &'a Node<'static>,
        object: &'a Node<'static>
    ) -> TripleView<'a> {
        TripleView { graph, subject, predicate, object }
    }
}

impl<'a> WriteNQuads for TripleView<'a> {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.subject.write_nquads(writer)?;
        writer.write_all(b" ")?;
        self.predicate.write_nquads(writer)?;
        writer.write_all(b" ")?;
        self.object.write_nquads(writer)?;

        if let Some(graph) = self.graph {
            writer.write_all(b" ")?;
            graph.write_nquads(writer)?;
        }

        writer.write_all(b" .\n")?;
        
        Ok(())
    }
}