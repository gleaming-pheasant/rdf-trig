use std::borrow::Cow;


use crate::namespaces::{Namespace, NamespaceId};
use crate::nodes::{Object, Predicate, Subject, StagingNode};
use crate::traits::ToInterned;

/// An `IriNode` is composed of a [`Namespace`] (to allow assigning the iri to a 
/// shared iri using a `prefix`) and an `endpoint`.
/// 
/// An `IriNode` can be used as a [`Subject`], [`Predicate`] or [`Object`] so 
/// this struct implements [`Into`] for all three node varieties.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct IriNode<'a> {
    namespace: Namespace<'a>,
    endpoint: Cow<'a, str>
}

impl<'a> IriNode<'a> {
    /// Create a new [`IriNode`].
    pub(crate) fn new<C: Into<Cow<'a, str>>>(
        namespace: Namespace<'a>, endpoint: C
    ) -> IriNode<'a> {
        IriNode { namespace, endpoint: endpoint.into() }
    }

    /// Allows you to create a new `IriNode` which is composed of static values 
    /// known at compile time, exported via [`Predicate`](crate::nodes::Predicate).
    pub(crate) const fn new_const(
        namespace: Namespace<'static>, endpoint: &'static str
    ) -> IriNode<'static> {
        IriNode { namespace, endpoint: Cow::Borrowed(endpoint) }
    }

    /// Consume this `IriNode`, returning a tuple of its `namespace` and 
    /// `endpoint`.
    pub(crate) fn into_parts(self) -> (Namespace<'a>, Cow<'a, str>) {
        (self.namespace, self.endpoint)
    }
}

impl<'a> Into<Object<'a>> for IriNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Iri(self)
    }
}

impl<'a> Into<Predicate<'a>> for IriNode<'a> {
    #[inline]
    fn into(self) -> Predicate<'a> {
        Predicate::new(self)
    }
}

impl<'a> Into<Subject<'a>> for IriNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Iri(self)
    }
}

/// An [`IriNode`] that stores an already interned 
/// [`Namespace`](crate::namespaces::Namespace)'s `NamespaceId`.
/// 
/// This type still retains its lifetime, as it can still reference a temporary 
/// value prior to the interning of the node itself.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StagingIriNode<'a> {
    namespace_id: NamespaceId,
    endpoint: Cow<'a, str>
}

impl<'a> StagingIriNode<'a> {
    /// Create a new `StagingIriNode` from a retrieved [`NamespaceId`] and an 
    /// `endpoint`.
    pub(crate) fn new(
        namespace_id: NamespaceId, endpoint: Cow<'a, str>
    ) -> StagingIriNode<'a> {
        StagingIriNode { namespace_id, endpoint }
    }

    /// Get the `namespace_id` for this `StagingIriNode`.
    pub(crate) fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    /// Get a reference to the `endpoint` for this `StagingIriNode`.
    pub(crate) fn endpoint(&self) -> &str {
        &self.endpoint
    }

    // pub(crate) fn to_interned(&self) -> InternedIriNode {
    //     InternedIriNode(
    //         StagingIriNode {
    //             namespace_id: self.namespace_id.clone(),
    //             endpoint: Cow::Owned(self.endpoint.to_string())
    //         }
    //     )
    // }
}

impl<'a> ToInterned for StagingIriNode<'a> {
    type InternedType = StagingIriNode<'static>;

    #[inline]
    fn to_interned(&self) -> Self::InternedType {
        StagingIriNode {
            namespace_id: self.namespace_id(),
            endpoint: Cow::Owned(self.endpoint.clone().into_owned())
        }
    }
}

impl<'a> Into<StagingNode<'a>> for StagingIriNode<'a> {
    /// Wrap this `Staging` as a `StagedNode` in preparation for interning.
    #[inline]
    fn into(self) -> StagingNode<'a> {
        StagingNode::Iri(self)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedIriNode(pub StagingIriNode<'static>);