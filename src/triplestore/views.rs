//! Contains views into borrowed 'static objects that can be retrieved from a 
//! [`TripleStore`] and its component parts.
//! 
use std::io::{self, Write};

use crate::{WriteTriG, nodes::Node};

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
    subject: &'a Node<'static>,
    predicate: &'a Node<'static>,
    object: &'a Node<'static>
}

impl<'a> TripleView<'a> {
    /// A private constructor for simple declaration of a `TripleView`, composed 
    /// of parts retrieved from a `TripleStore`.
    pub(crate) fn new(
        subject: &'a Node<'static>, predicate: &'a Node<'static>,
        object: &'a Node<'static>
    ) -> TripleView<'a> {
        TripleView { subject, predicate, object }
    }
}

impl<'a> WriteTriG for TripleView<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.subject.write_trig(writer)?;
        writer.write_all(b" ")?;
        self.predicate.write_trig(writer)?;
        writer.write_all(b" ")?;
        self.object.write_trig(writer)?;
        writer.write_all(b" .\n")?;

        Ok(())
    }
}