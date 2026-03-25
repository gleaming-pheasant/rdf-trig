//! Contains views into borrowed 'static objects that can be retrieved from a 
//! [`TripleStore`](crate::triplestore::TripleStore) and its component parts.
use std::io::{self, Write};

use crate::Namespace;
use crate::namespaces::statics::RDF;
use crate::traits::WriteTriG;
use crate::nodes::{BlankNode, LiteralNode};
use crate::utils::{write_escaped_local_name, write_escaped_url_component};

/// A temporary view to one of [`BlankNode`], [`IriNodeView`] or [`LiteralNode`], 
/// which must be constructed by resolving data from a `TripleStore`.
#[derive(Debug)]
pub(crate) enum NodeView<'a> {
    Blank(&'a BlankNode<'a>),
    Iri(IriNodeView<'a>),
    Literal(&'a LiteralNode<'a>)
}

impl<'a> WriteTriG for NodeView<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            NodeView::Blank(blank) => blank.write_trig(writer),
            NodeView::Iri(iri) => iri.write_trig(writer),
            NodeView::Literal(literal) => literal.write_trig(writer)
        }
    }
}

/// A temporary view to an [`IriNode`](crate::nodes::IriNode) which must be 
/// constructed by resolving data from a `TripleStore`, first resolving the 
/// `prefix` from its interned [`Namespace`](crate::namespaces::Namespace).
#[derive(Debug)]
pub(crate) struct IriNodeView<'a> {
    namespace: &'a Namespace<'a>,
    local_name: &'a str
}

impl<'a> IriNodeView<'a> {
    /// Create a new `IriNodeView` from parts retrieved by querying a 
    /// `DataStore`.
    pub fn new(
        namespace: &'a Namespace<'a>, local_name: &'a str
    ) -> IriNodeView<'a> {
        IriNodeView { namespace, local_name }
    }
}

impl WriteTriG for IriNodeView<'_> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if *self.namespace == RDF && self.local_name == "type" {
            return writer.write_all(b"a")
        }

        // Can be printed as a local_name
        if self.local_name.bytes() // Check they're printable as a local_name.
        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') {
            write_escaped_local_name(writer, self.namespace.prefix())?;
            writer.write_all(b":")?;
            return write_escaped_local_name(writer, self.local_name)
        }

        // Otherwise, output as full IRI
        writer.write_all(b"<")?;
        writer.write_all(self.namespace.iri().as_bytes())?; // Guaranteed valid.
        write_escaped_url_component(writer, self.local_name)?;
        writer.write_all(b">")?;

        Ok(())
    }
}

/// A temporary view to a [`Graph`](crate::nodes::Graph) which must be 
/// constructed by resolving data from a `TripleStore`.
/// 
/// This is distinct from an [`IriNodeView`]; it implements [`WriteTriG`] by 
/// wrapping the full URI in `<` `>`, to guarantee formatting on insertion to a 
/// static triple store.
#[derive(Debug)]
pub(crate) struct GraphView<'a> {
    namespace_iri: &'a str,
    local_name: &'a str
}

impl<'a> GraphView<'a> {
    /// Create a new `GraphView` from parts retrieved by querying a 
    /// `DataStore`.
    pub fn new(
        namespace_iri: &'a str, local_name: &'a str
    ) -> GraphView<'a> {
        GraphView { namespace_iri, local_name }
    }
}

impl WriteTriG for GraphView<'_> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"<")?;
        writer.write_all(self.namespace_iri.as_bytes())?;
        write_escaped_url_component(writer, self.local_name)?;
        writer.write_all(b">")?;

        Ok(())
    }
}

/// A temporary view to a [`Triple`](crate::triples::Triple).
/// 
/// It contains a reference to a [`Node`]s for the `subject`, `predicate` and 
/// `object` components of the triple.
/// 
/// It can only be created through a [`TripleStore`](crate::triplestore::TripleStore) 
/// in order to retrieve any interned [`Namespace`]s for [`IriNode`]s.
/// 
/// `TripleView` implements [`WriteTriG`] for writing individual triples in TriG 
/// format. This is done without a named graph, so each triple is implicitly 
/// added to a default graph.
/// 
/// It does not retrieve the [`Node`] for a related `Graph`; this must be 
/// handled separately, using the `GraphIndex` of the `TripleStore` in order to 
/// prevent writing the graph declaration for every `Triple` in the store.
#[derive(Debug)]
pub struct TripleView<'a> {
    subject: NodeView<'a>,
    predicate: NodeView<'a>,
    object: NodeView<'a>
}

impl<'a> TripleView<'a> {
    /// A private constructor for simple declaration of a `TripleView`, composed 
    /// of parts retrieved from a `TripleStore`.
    pub(crate) fn new(
        subject: NodeView<'a>, predicate: NodeView<'a>,
        object: NodeView<'a>
    ) -> TripleView<'a> {
        TripleView { subject, predicate, object }
    }
}

impl WriteTriG for TripleView<'_> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.subject.write_trig(writer)?;
        writer.write_all(b" ")?;
        self.predicate.write_trig(writer)?;
        writer.write_all(b" ")?;
        self.object.write_trig(writer)?;

        Ok(())
    }
}