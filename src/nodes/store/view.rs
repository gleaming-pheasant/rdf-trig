use crate::namespaces::Namespace;

/// `IriNodeView` contains references to an [`IriNode`]'s interned [`Namespace`] 
/// and its `endpoint` and, like other `...View` structs in this crate, is 
/// useful for representing interned data.
/// 
/// `IriNodeView` implements [`WriteTriG`] for writing the shortform IRI 
/// ("{namespace_prefix}:{endpoint}") for display in TriG format.
#[derive(Debug)]
pub struct IriNodeView<'a> {
    namespace: &'a Namespace<'a>,
    endpoint: &'a str
}

impl<'a> IriNodeView<'a> {
    pub(crate) fn new(
        namespace: &'a Namespace<'a>, endpoint: &'a str
    ) -> IriNodeView<'a> {
        IriNodeView { namespace, endpoint }
    }
}


/// A `NodeView` is a reference to an expanded "node". [`BlankNode`]s and 
/// [`LiteralNode`]s remain just references, while an [`IriNode`] becomes an 
/// [`IriNodeView`] (containing a reference to an interned [`Namespace`]).
/// 
/// `NodeView` implements [`WriteTriG`] for outputting the "node" in TriG 
/// format.
#[derive(Debug)]
pub enum NodeView<'a> {
    Blank(&'a BlankNode),
    Iri(IriNodeView<'a>),
    Literal(&'a LiteralNode)
}

impl<'a> WriteTriG for NodeView<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        match self {
            NodeView::Blank(blank) => blank.write_trig(writer),
            NodeView::Iri(iri) => iri.write_trig(writer),
            NodeView::Literal(literal) => literal.write_trig(writer)
        }
    }
}

impl<'a> WriteTriG for IriNodeView<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        write_escaped_local_name(writer, self.namespace.prefix())?;
        writer.write_all(b":")?;
        write_escaped_url_component(writer, self.endpoint)?;
        Ok(())
    }
}


// impl WriteTriG for InternedLiteralNode {
//     fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
//         match self {
//             InternedLiteralNode::Boolean(boolean) => boolean.write_trig(writer),
//             InternedLiteralNode::DateTime(datetime) => datetime.write_trig(writer),
//             InternedLiteralNode::Decimal(decimal) => decimal.write_trig(writer),
//             InternedLiteralNode::GYear(gyear) => gyear.write_trig(writer),
//             InternedLiteralNode::LangString { value, language } => {

//             },
//             InternedLiteralNode::String(string) => {
                
//             }
//         }
//     }
// }


// impl WriteTriG for LangStringLiteral<'_> {
//     fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
//         writer.write_all(b"\"")?;
//         write_escaped_literal(writer, &self.value)?;
//         writer.write_all(b"\"@")?;
//         writer.write_all(&self.language.as_bytes())?;

//         Ok(())
//     }
// }

// impl WriteTriG for StringLiteral<'_> {
//     fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
//         writer.write_all(b"\"")?;
//         write_escaped_literal(writer, &self.0)?;
//         writer.write_all(b"\"")?;

//         Ok(())
//     }
// }