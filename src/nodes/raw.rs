use std::borrow::Cow;
use std::io::{Result as IoResult, Write};

use crate::namespaces::{Namespace, NamespaceId};
use crate::traits::WriteTriG;

/// An `IriNode` is composed of a [`Namespace`] (to allow assigning the iri to a 
/// shared iri using a `prefix`) and an `endpoint`.
/// 
/// These must be instantiated with the [`Subject`], [`Predicate`] or [`Object`] 
/// types directly, to prevent invalid nodes being used in the wrong locations 
/// in a [`Triple`](crate::groups::triples::Triple).
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct IriNode {
    namespace: Namespace,
    endpoint: Cow<'static, str>
}

impl IriNode {
    /// Create a new [`IriNode`].
    pub(crate) fn new<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> IriNode {
        IriNode { namespace, endpoint: endpoint.into() }
    }

    pub(crate) fn new_with_new_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> IriNode
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>
    {
        IriNode {
            namespace: Namespace::new(prefix, iri),
            endpoint: endpoint.into()
        }
    }

    /// Allows you to create a new `IriNode` which is composed of static values 
    /// known as compile time, exported via [`Predicate`](crate::nodes::Predicate).
    pub(crate) const fn new_const(
        namespace: Namespace, endpoint: &'static str
    ) -> IriNode {
        IriNode { namespace, endpoint: Cow::Borrowed(endpoint) }
    }

    /// Consume this `IriNode`, returning a tuple of its `namespace` and 
    /// `endpoint`.
    pub(crate) fn into_parts(self) -> (Namespace, Cow<'static, str>) {
        (self.namespace, self.endpoint)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedIriNode {
    namespace_id: NamespaceId,
    endpoint: Cow<'static, str>
}

impl InternedIriNode {
    /// Create a new [`InternedIriNode`].
    pub(crate) fn new(
        namespace_id: NamespaceId, endpoint: Cow<'static, str>
    ) -> InternedIriNode {
        InternedIriNode { namespace_id, endpoint }
    }

    /// Get the `namespace_id` for this `InternedIriNode`.
    pub(crate) fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    /// Get a reference to the `endpoint` for this `InternedIriNode`.
    pub(crate) fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// A `BlankNode` is a standard RDF blank node. It serves as a a place to store 
/// known facts about a resource within a graph, without knowing the resource's 
/// specific IRI.
/// 
/// `BlankNode` directly implements [`WriteTriG`], prefixing the provided id 
/// with the standard blank node "_:" prefix.
/// 
/// `BlankNode`s cannot be initialised directly, and must be generated as part 
/// of [`Subject`] or [`Object`] constructors.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BlankNode(Cow<'static, str>);

impl BlankNode {
    /// Create a new `BlankNode` with the provided `id`.
    pub(crate) fn new<C: Into<Cow<'static, str>>>(id: C) -> BlankNode {
        BlankNode(id.into())
    }
}

impl WriteTriG for BlankNode {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(b"_:")?;
        writer.write_all(self.0.as_bytes())
    }
}

/// A `LiteralNode` is an enumerator over xsd literal types, such as "strings" 
/// (with optional language tags), "datetimes" and "gYears".
/// 
/// Because there is nothing to explicitly intern in a `LiteralNode`, this type 
/// directly implements the [`WriteTriG`] trait for TriG formatting.
/// 
/// This enum is __non_exhaustive__, with additional types not currently on the 
/// roadmap.
#[derive(Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LiteralNode {
    Datetime(Cow<'static, str>),
    GYear(Cow<'static, str>),
    String(StringLiteral)
}

impl LiteralNode {
    pub(crate) fn new_datetime() {
        todo!()
    }

    pub(crate) fn new_datetime_unchecked() {
        todo!()
    }

    pub(crate) fn new_gyear() {
        todo!()
    }
    
    pub(crate) fn new_gyear_unchecked() {
        todo!()
    }

    /// Create a new `LiteralNode::String` with the provided `language` and 
    /// string `value`.
    pub(crate) fn string<L, V>(language: Option<L>, value: V) -> LiteralNode 
    where
        L: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>
    {
        LiteralNode::String(StringLiteral::new(language, value))
    }

    /// Create a new `LiteralNode::String` with the `language` code already set 
    /// to "en" for English.
    pub(crate) fn string_en<V: Into<Cow<'static, str>>>(
        value: V
    ) -> LiteralNode {
        LiteralNode::String(StringLiteral::new_en(value))
    }

    /// Create a new `LiteralNode::String` with the `language` code set to 
    /// `None`.
    pub(crate) fn string_no_lang<V: Into<Cow<'static, str>>>(
        value: V
    ) -> LiteralNode {
        LiteralNode::String(StringLiteral::new_no_lang(value))
    }
}

impl WriteTriG for &LiteralNode {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        match self {
            LiteralNode::Datetime(dt) => {
                writer.write_all(b"\"")?;
                writer.write_all(dt.as_bytes())?;
                writer.write_all(b"\"^^xsd:dateTime")?;
            },
            LiteralNode::GYear(gy) => {
                writer.write_all(b"\"")?;
                writer.write_all(gy.as_bytes())?;
                writer.write_all(b"\"^^xsd:gYear")?;
            },
            LiteralNode::String(st) => {
                match st.language() {
                    Some(lang) => {
                        write!(
                            writer, "\"{}\"@{}",
                            st.value(), lang
                        )?;},
                    None => {
                        writer.write_all(b"\"")?;
                        writer.write_all(st.value().as_bytes())?;
                        writer.write_all(b"\"")?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct StringLiteral {
    language: Option<Cow<'static, str>>,
    value: Cow<'static, str>
}

impl StringLiteral {
    /// Create a new `StringLiteral` from a `language` tag and `value`.
    pub(crate) fn new<L, V>(language: Option<L>, value: V) -> StringLiteral 
    where
        L: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>
    {
        StringLiteral {
            language: language.map(|l| l.into()),
            value: value.into()
        }
    }

    /// Create a new `StringLiteral` with the `language` set to Some("en").
    pub(crate) fn new_en<V: Into<Cow<'static, str>>>(value: V) -> StringLiteral {
        StringLiteral { language: Some("en".into()), value: value.into() }
    }

    /// Create a new `StringLiteral` with the `language` set to `None`.
    pub(crate) fn new_no_lang<V: Into<Cow<'static, str>>>(
        value: V
    ) -> StringLiteral {
        StringLiteral { language: None, value: value.into() }
    }

    /// Return a reference to this `StringLiteral`'s `language`.
    pub(crate) fn language(&self) -> &Option<Cow<'static, str>> {
        &self.language
    }

    /// Return a reference to this `StringLiteral`'s `value`.
    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum InternedNode {
    Blank(BlankNode),
    Iri(InternedIriNode),
    Literal(LiteralNode)
}