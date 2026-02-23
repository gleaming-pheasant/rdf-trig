use std::borrow::Cow;
use std::io::{Result as IoResult, Write};
use std::ops::Deref;

use crate::FastIndexSet;
use crate::namespaces::{Namespace, NamespaceId};
use crate::traits::WriteTriG;


/// A `Subject` is an enumerator over the two valid RDF "node" types for 
/// subjects; blank nodes, and IRI nodes.
#[derive(Debug)]
pub enum Subject {
    Blank(BlankNode),
    Iri(IriNode)
}

impl Subject {
    /// Create a new `Subject::Blank` node, with `id` being the identifier for 
    /// the blank resource.
    pub fn blank<C: Into<Cow<'static, str>>>(id: C) -> Subject {
        Subject::Blank(BlankNode(id.into()))
    }

    /// Create a new `Subject::Iri` node with a given [`Namespace`] and 
    /// `endpoint` [str].
    pub fn iri<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> Subject {
        Subject::Iri(IriNode::new(namespace, endpoint))
    }
    
    /// Create a new `Subject::Iri` node, simultaneously declaring a new 
    /// [`Namespace`] from `prefix` and `iri` [str] values.
    pub fn iri_with_new_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> Subject
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>
    {
        Subject::Iri(IriNode::new_with_new_namespace(prefix, iri, endpoint))
    }
}


/// A `Predicate` is simply a wrapper around an [`IriNode`], as this is the only 
/// valid RDF resource type for a predicate.
#[derive(Debug)]
pub struct Predicate {
    iri: IriNode
}

impl Predicate {
    /// Create a new `Predicate`, with a pre-built [`Namespace`] and an 
    /// `endpoint` [str].
    pub fn new<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> Predicate {
        Predicate {
            iri: IriNode::new(namespace, endpoint)
        }
    }

    /// Create a new `Predicate` and simultaneously build a new [`Namespace`] 
    /// from provided `prefix` and `iri` [str]s.
    pub fn new_with_new_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> Predicate
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>
    {
        Predicate {
            iri: IriNode::new_with_new_namespace(prefix, iri, endpoint)
        }
    }

    /// Consume this `Predicate`, returning the contained [`Namespace`] and 
    /// `endpoint`.
    pub fn into_parts(self) -> (Namespace, Cow<'static, str>) {
        self.iri.into_parts()
    }
}


/// An `Object` provides wrappers around the three main RDF node types: blank 
/// nodes, iri nodes and literal nodes.
#[derive(Debug)]
pub enum Object {
    Blank(BlankNode),
    Iri(IriNode),
    Literal(LiteralNode)
}

impl Object {
    /// Create a new `Object::Blank` with the provided `id` as the name of the 
    /// blank resource.
    pub fn blank<C: Into<Cow<'static, str>>>(id: C) -> Object {
        Object::Blank(BlankNode(id.into()))
    }

    /// Create a new `Object::Iri` from a provided [`Namespace`] and `endpoint` 
    /// [str].
    pub fn iri<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> Object {
        Object::Iri(IriNode::new(namespace, endpoint))
    }

    /// Create a new `Object::Iri` and simultaneously create a new [`Namespace`] 
    /// from the provided `prefix` and `iri` [str]s.
    pub fn iri_with_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> Object
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>
    {
        Object::Iri(IriNode::new_with_new_namespace(prefix, iri, endpoint))
    }

    /// Create a new `Object::Literal` as a [`StringLiteral`] with the 
    /// `language` tag set to "en".
    pub fn string_en<C: Into<Cow<'static, str>>>(value: C) -> Object {
        Object::Literal(LiteralNode::string_en(value))
    }

    /// Create a new `Object::Literal` as a [`StringLiteral`] with the 
    /// `language` set to `None`.
    pub fn string_no_lang<C: Into<Cow<'static, str>>>(value: C) -> Object {
        Object::Literal(LiteralNode::string_no_lang(value))
    }
}


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


impl WriteTriG for &BlankNode {
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
    pub(crate) fn string<L, V>(language: Option<L>, value: V) -> LiteralNode 
    where
        L: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>
    {
        LiteralNode::String(StringLiteral::new(language, value))
    }

    pub(crate) fn string_en<V: Into<Cow<'static, str>>>(
        value: V
    ) -> LiteralNode {
        LiteralNode::String(StringLiteral::new_en(value))
    }

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
                            writer, "\"{}@{}\"",
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct NodeId(u32);

impl NodeId {
    pub(crate) fn from(ix: usize) -> NodeId {
        debug_assert!(ix <= u32::MAX as usize);
        NodeId(ix as u32)
    }
}

impl Deref for NodeId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A wrapper around an [`IndexSet<InternedNode>`] which serves to store unique 
/// "nodes" and hand out [`NodeId`]s as references to the [`InternedNode`]s.
#[derive(Debug)]
pub(crate) struct NodeStore {
    store: FastIndexSet<InternedNode>
}

impl NodeStore {
    /// Create a new [`NodeStore`].
    pub(crate) fn new() -> NodeStore {
        NodeStore { store: FastIndexSet::default() }
    }

    /// Add an `InternedNode` to this `NodeStore`, returning a `NodeId` (a 
    /// wrapped `IndexSet` index cast as u32).
    pub(crate) fn intern_node(&mut self, node: InternedNode) -> NodeId {
        NodeId::from(self.store.insert_full(node).0)
    }

    /// Retrieve an `InternedNode` from the provided `NodeId`.
    pub(crate) fn query_node(&self, node_id: NodeId) -> &InternedNode {
        self.store.get_index(*node_id as usize).unwrap()
    }
}

/// `IriNodeView` contains references to an [`IriNode`]'s interned [`Namespace`] 
/// and its `endpoint` and, like other `...View` structs in this crate, is 
/// useful for representing interned data.
/// 
/// `IriNodeView` implements [`WriteTriG`] for writing the shortform IRI 
/// ("{namespace_prefix}:{endpoint}") for display in TriG format.
#[derive(Debug)]
pub struct IriNodeView<'a> {
    namespace: &'a Namespace,
    endpoint: &'a str
}

impl<'a> IriNodeView<'a> {
    pub(crate) fn new(
        namespace: &'a Namespace, endpoint: &'a str
    ) -> IriNodeView<'a> {
        IriNodeView { namespace, endpoint }
    }
}


impl<'a> WriteTriG for IriNodeView<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        write!(writer, "{}:{}", self.namespace.prefix(), self.endpoint)
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