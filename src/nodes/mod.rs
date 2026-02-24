use std::borrow::Cow;
use std::io::{Result as IoResult, Write};
use std::ops::Deref;

use crate::FastIndexSet;
use crate::namespaces::Namespace;
use crate::traits::WriteTriG;

pub mod raw;

use raw::{BlankNode, IriNode, LiteralNode, InternedNode};

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
        Subject::Blank(BlankNode::new(id))
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
        Object::Blank(BlankNode::new(id))
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