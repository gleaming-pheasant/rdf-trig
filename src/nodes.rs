use std::borrow::Cow;
use std::ops::Deref;

use crate::FastIndexSet;
use crate::namespaces::{Namespace, NamespaceId};

#[derive(Debug)]
pub enum Subject {
    Blank(BlankNode),
    Iri(IriNode)
}

impl Subject {
    pub fn blank<C: Into<Cow<'static, str>>>(id: C) -> Subject {
        Subject::Blank(BlankNode(id.into()))
    }

    pub fn iri<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> Subject {
        Subject::Iri(IriNode::new(namespace, endpoint))
    }

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


#[derive(Debug)]
pub enum Predicate {
    Iri(IriNode)
}

impl Predicate {
    pub fn iri<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> Predicate {
        Predicate::Iri(IriNode::new(namespace, endpoint))
    }

    pub fn iri_with_new_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> Predicate
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>
    {
        Predicate::Iri(IriNode::new_with_new_namespace(prefix, iri, endpoint))
    }
}


#[derive(Debug)]
pub enum Object {
    Blank(BlankNode),
    Iri(IriNode),
    Literal(LiteralNode)
}

impl Object {
    pub fn blank<C: Into<Cow<'static, str>>>(id: C) -> Object {
        Object::Blank(BlankNode(id.into()))
    }

    pub fn iri<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> Object {
        Object::Iri(IriNode::new(namespace, endpoint))
    }

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

    pub fn string_en<C: Into<Cow<'static, str>>>(value: C) -> Object {
        Object::Literal(LiteralNode::string_en(value))
    }

    pub fn string_no_lang<C: Into<Cow<'static, str>>>(value: C) -> Object {
        Object::Literal(LiteralNode::string_no_lang(value))
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct IriNode {
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

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct BlankNode(Cow<'static, str>);

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum LiteralNode {
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

    pub(crate) fn string_en<V: Into<Cow<'static, str>>>(value: V) -> LiteralNode {
        LiteralNode::String(StringLiteral::new_en(value))
    }

    pub(crate) fn string_no_lang<V: Into<Cow<'static, str>>>(value: V) -> LiteralNode {
        LiteralNode::String(StringLiteral::new_no_lang(value))
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StringLiteral {
    language: Option<Cow<'static, str>>,
    value: Cow<'static, str>
}

impl StringLiteral {
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

    pub(crate) fn new_en<V: Into<Cow<'static, str>>>(value: V) -> StringLiteral {
        StringLiteral { language: Some("en".into()), value: value.into() }
    }

    pub(crate) fn new_no_lang<V: Into<Cow<'static, str>>>(value: V) -> StringLiteral {
        StringLiteral { language: None, value: value.into() }
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