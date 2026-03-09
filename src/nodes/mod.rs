use std::borrow::Cow;
use std::io::{Result as IoResult, Write};
use std::ops::Deref;

use crate::FastIndexSet;
use crate::errors::RdfTrigError;
use crate::namespaces::Namespace;
use crate::traits::WriteTriG;
use crate::utils::{write_escaped_local_name, write_escaped_url_component};

pub mod raw;

use raw::{BlankNode, IriNode, LiteralNode, InternedNode};


/// A `Subject` is an enumerator over the two valid RDF "node" types for 
/// subjects; blank nodes, and IRI nodes.
/// 
/// As with [`Graph`](crate::graphs::Graph)s, a `Subject::Iri` is a combination 
/// of a [`Namespace`] and an `endpoint`.
/// 
/// See [`crate`] documentation for details on this crates relationship with 
/// IRIs.
#[derive(Debug)]
pub enum Subject<'a> {
    Blank(BlankNode<'a>),
    Iri(IriNode<'a>)
}

impl<'a> Subject<'a> {
    /// Create a new `Subject::Blank` node, with `id` being the identifier for 
    /// the blank resource.
    pub fn blank<C: Into<Cow<'a, str>>>(id: C) -> Subject<'a> {
        Subject::Blank(BlankNode::new(id))
    }

    /// Create a new `Subject::Iri` node with a given [`Namespace`] and 
    /// `endpoint` [str].
    pub fn iri<C: Into<Cow<'a, str>>>(
        namespace: Namespace<'a>, endpoint: C
    ) -> Subject<'a> {
        Subject::Iri(IriNode::new(namespace, endpoint))
    }
    
    /// Create a new `Subject::Iri` node, simultaneously declaring a new 
    /// [`Namespace`] from `prefix` and `iri` [str] values.
    /// Returns a [`RdfTrigError::InvalidIri`] if the `iri` for the `Namespace` 
    /// is invalid.
    pub fn iri_with_new_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> Result<Subject<'a>, RdfTrigError<'a>>
    where
        P: Into<Cow<'a, str>>,
        I: Into<Cow<'a, str>>,
        C: Into<Cow<'a, str>>
    {
        Ok(Subject::Iri(
            IriNode::new_with_new_namespace(prefix, iri, endpoint)?
        ))
    }
}


/// A `Predicate` is simply a wrapper around an [`IriNode`], as this is the only 
/// valid RDF resource type for a predicate.
/// 
/// As with [`Graph`](crate::graphs::Graph)s, a `Predicate` is a combination 
/// of a [`Namespace`] and an `endpoint`.
/// 
/// See [`crate`] documentation for details on this crates relationship with 
/// IRIs.
#[derive(Debug)]
pub struct Predicate<'a> {
    iri: IriNode<'a>
}

impl<'a> Predicate<'a> {
    /// Create a new `Predicate`, with a pre-built [`Namespace`] and an 
    /// `endpoint` [str].
    pub fn new<C: Into<Cow<'a, str>>>(
        namespace: Namespace<'a>, endpoint: C
    ) -> Predicate<'a> {
        Predicate {
            iri: IriNode::new(namespace, endpoint)
        }
    }

    /// Create a new `Predicate` and simultaneously build a new [`Namespace`] 
    /// from provided `prefix` and `iri` [str]s.
    /// 
    /// Returns a [`RdfTrigError::InvalidIri`] if the `iri` for the `Namespace` 
    /// is invalid.
    pub fn new_with_new_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> Result<Predicate<'a>, RdfTrigError<'a>>
    where
        P: Into<Cow<'a, str>>,
        I: Into<Cow<'a, str>>,
        C: Into<Cow<'a, str>>
    {
        Ok(Predicate {
            iri: IriNode::new_with_new_namespace(prefix, iri, endpoint)?
        })
    }

    /// Consume this `Predicate`, returning the contained [`Namespace`] and 
    /// `endpoint`.
    pub fn into_parts(self) -> (Namespace<'a>, Cow<'a, str>) {
        self.iri.into_parts()
    }

    /// Allows you to declare a `Predicate` using a `Namespace` and endpoint 
    /// known at compile time.
    /// 
    /// Useful if you know a Predicate will be regularly used.
    pub const fn new_const(
        namespace: Namespace<'static>, endpoint: &'static str
    ) -> Predicate<'a> {
        Predicate { iri: IriNode::new_const(namespace, endpoint) }
    }
}


/// An `Object` provides wrappers around the three main RDF node types: blank 
/// nodes, iri nodes and literal nodes.
/// 
/// As with [`Graph`](crate::graphs::Graph)s, an `Object::Iri` is a combination 
/// of a [`Namespace`] and an `endpoint`.
/// 
/// See [`crate`] documentation for details on this crates relationship with 
/// IRIs.
#[derive(Debug)]
pub enum Object<'a> {
    Blank(BlankNode<'a>),
    Iri(IriNode<'a>),
    Literal(LiteralNode<'a>)
}

impl<'a> Object<'a> {
    /// Create a new `Object::Blank` with the provided `id` as the name of the 
    /// blank resource.
    pub fn blank<C: Into<Cow<'a, str>>>(id: C) -> Object<'a> {
        Object::Blank(BlankNode::new(id))
    }

    /// Create a new `Object::Iri` from a provided [`Namespace`] and `endpoint` 
    /// [str].
    pub fn iri<C: Into<Cow<'a, str>>>(
        namespace: Namespace<'a>, endpoint: C
    ) -> Object<'a> {
        Object::Iri(IriNode::new(namespace, endpoint))
    }

    /// Create a new `Object::Iri` and simultaneously create a new [`Namespace`] 
    /// from the provided `prefix` and `iri` [str]s.
    /// 
    /// Returns a [`RdfTrigError::InvalidIri`] if the `iri` for the `Namespace` 
    /// is invalid.
    pub fn iri_with_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> Result<Object<'a>, RdfTrigError<'a>>
    where
        P: Into<Cow<'a, str>>,
        I: Into<Cow<'a, str>>,
        C: Into<Cow<'a, str>>
    {
        Ok(Object::Iri(
            IriNode::new_with_new_namespace(prefix, iri, endpoint)?
        ))
    }

    /// Create a new `Object::Literal` string type with the provided `language` 
    /// and `value`.
    /// 
    /// Returns an `RdfTrigError::InvalidLanguage` if the provided `language` is 
    /// not a valid ISO-639 language code.
    pub fn string<L, C>(
        language: Option<L>, value: C
    )-> Result<Object<'a>, RdfTrigError<'a>>
    where
        L: Into<Cow<'a, str>>,
        C: Into<Cow<'a, str>>
    {
        Ok(Object::Literal(LiteralNode::string(language, value)?))
    }

    /// Create a new `Object::Literal` string type with the `language` tag set 
    /// to "en".
    pub fn string_en<C: Into<Cow<'a, str>>>(value: C) -> Object<'a> {
        Object::Literal(LiteralNode::string_en(value))
    }

    /// Create a new `Object::Literal` string type with the `language` set to 
    /// `None`.
    pub fn string_no_lang<C: Into<Cow<'a, str>>>(value: C) -> Object<'a> {
        Object::Literal(LiteralNode::string_no_lang(value))
    }

    /// Create a new `Object::Literal` boolean type from a Rust native [`bool`].
    pub fn boolean_from_native(value: bool) -> Object<'a> {
        Object::Literal(LiteralNode::from(value))
    }

    /// Create a new `Object::Literal` boolean type from the given `value`.
    /// 
    /// Returns an `RdfTrigError::InvalidBoolean` if the provided `value` is not 
    /// "true", "false", "1" or "0".
    pub fn boolean_from_str<C: Into<Cow<'a, str>>>(value: C)
    -> Result<Object<'a>, RdfTrigError<'a>> {
        Ok(Object::Literal(LiteralNode::boolean(value)?))
    }

    /// Create a new `Object::Literal` datetime type from the given `value`.
    /// 
    /// Returns an `RdfTrigError::InvalidDateTime` if the provided `value` 
    /// cannot be parsed as an XML Schema dateTime.
    /// 
    /// This is an awkward non-ISO specification, but allows datetimes both with 
    /// or without timezone identifiers.
    pub fn datetime<C: Into<Cow<'a, str>>>(value: C)
    -> Result<Object<'a>, RdfTrigError<'a>> {
        Ok(Object::Literal(LiteralNode::datetime(value)?))
    }
    
    #[cfg(feature = "time")]
    /// Only on the `time` feature.
    /// 
    /// Converts the provided [`time::PrimitiveDateTime`] into an 
    /// `Object::Literal`, but fails if the provided value would return a 
    /// [`time::error::Format`].
    pub fn datetime_from_time_primitive(value: time::PrimitiveDateTime)
    -> Result<Object<'a>, RdfTrigError<'a>> {
        Ok(Object::Literal(LiteralNode::try_from(value)?))
    }

    #[cfg(feature = "time")]
    /// Only on the `time` feature.
    /// 
    /// Converts the provided [`time::OffsetDateTime`] into an 
    /// `Object::Literal`, but fails if the provided value would return a 
    /// [`time::error::Format`].
    pub fn datetime_from_time_offset(value: time::OffsetDateTime)
    -> Result<Object<'a>, RdfTrigError<'a>> {
        Ok(Object::Literal(LiteralNode::try_from(value)?))
    }

    #[cfg(feature = "chrono")]
    /// Only on the `chrono` feature.
    /// 
    /// Converts the provided [`chrono::NaiveDateTime`] into an 
    /// `Object::Literal` of type `DateTime`.
    pub fn datetime_from_chrono_naive(value: chrono::NaiveDateTime)
    -> Object<'a> {
        Object::Literal(LiteralNode::from(value))
    }

    #[cfg(feature = "chrono")]
    /// Only on the `chrono` feature.
    /// 
    /// Converts the provided [`chrono::DateTime<Utc>`] into an 
    /// `Object::Literal` of type `DateTime`.
    pub fn datetime_from_chrono_utc(value: chrono::DateTime<chrono::Utc>)
    -> Object<'a> {
        Object::Literal(LiteralNode::from(value))
    }

    #[cfg(feature = "chrono")]
    /// Only on the `chrono` feature.
    /// 
    /// Converts the provided [`chrono::DateTime<Local>`] into an 
    /// `Object::Literal` of type `DateTime`.
    pub fn datetime_from_chrono_local(value: chrono::DateTime<chrono::Local>)
    -> Object<'a> {
        Object::Literal(LiteralNode::from(value))
    }

    #[cfg(feature = "chrono")]
    /// Only on the `chrono` feature.
    /// 
    /// Converts the provided [`chrono::DateTime<FixedOffset>`] into an 
    /// `Object::Literal` of type `DateTime`.
    pub fn datetime_from_chrono_offset(value: chrono::DateTime<chrono::FixedOffset>)
    -> Object<'a> {
        Object::Literal(LiteralNode::from(value))
    }

    /// Create a new `Object::Literal` decimal type from the given `value`.
    /// 
    /// Returns an `RdfTrigError::InvalidDecimal` if the provided `value` cannot 
    /// be parsed as an f32.
    pub fn decimal_from_str<C: Into<Cow<'a, str>>>(value: C)
    -> Result<Object<'a>, RdfTrigError<'a>> {
        Ok(Object::Literal(LiteralNode::decimal(value)?))
    }

    /// Create a new `Object::Literal` decimal type from the provided [`f32`].
    pub fn decimal_from_native(value: f32) -> Object<'a> {
        Object::Literal(LiteralNode::from(value))
    }

    /// Create a new `Object::Literal` gYear type from the given `value`.
    /// 
    /// Returns an `RdfTrigError::InvalidGYear` if the provided `value` is not 
    /// in an XML Schema gYear format (it must be padded with 0s to be at least 
    /// 4 digits after an optional `-` sign and can have a timezone declaration).
    /// 
    /// Prioritise calling [`LiteralNode::gyear_from_i32`].
    pub fn gyear_from_str<C: Into<Cow<'a, str>>>(value: C)
    -> Result<Object<'a>, RdfTrigError<'a>> {
        Ok(Object::Literal(LiteralNode::gyear(value)?))
    }

    /// Create a new `Object::Literal` gYear from an [`i32`].
    /// 
    /// This will be stored as a valid, zero-padded gYear.
    pub fn gyear_from_i32(value: i32) -> Object<'a> {
        Object::Literal(LiteralNode::gyear_from_i32(value))
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
pub(crate) struct NodeStore<'a> {
    store: FastIndexSet<InternedNode<'a>>
}

impl<'a> NodeStore<'a> {
    /// Create a new [`NodeStore`].
    pub(crate) fn new() -> NodeStore<'a> {
        NodeStore { store: FastIndexSet::default() }
    }

    /// Add an `InternedNode` to this `NodeStore`, returning a `NodeId` (a 
    /// wrapped `IndexSet` index cast as u32).
    pub(crate) fn intern_node(&mut self, node: InternedNode<'a>) -> NodeId {
        NodeId::from(self.store.insert_full(node).0)
    }

    /// Retrieve an `InternedNode` from the provided `NodeId`.
    pub(crate) fn query_node(&self, node_id: NodeId) -> &InternedNode<'a> {
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


impl<'a> WriteTriG for IriNodeView<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        write_escaped_local_name(writer, self.namespace.prefix())?;
        writer.write_all(b":")?;
        write_escaped_url_component(writer, self.endpoint)?;
        Ok(())
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
    Blank(&'a BlankNode<'a>),
    Iri(IriNodeView<'a>),
    Literal(&'a LiteralNode<'a>)
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

#[cfg(test)]
mod tests {
    use super::Subject;

    #[test]
    fn test_add_invalid_namespace() {
        let subject = Subject::iri_with_new_namespace(
            "badOwl", "can't find owl schema", "Class"
        );

        assert!(subject.is_err());
    }
}