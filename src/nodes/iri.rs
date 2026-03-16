use std::borrow::Cow;

use crate::namespaces::{Namespace};
use crate::namespaces::store::{NamespaceId, NamespaceStore};
use crate::nodes::{Graph, Object, Predicate, StagingNode, Subject};
use crate::traits::ToStatic;

/// An `IriNode` is composed of a [`Namespace`] (to allow assigning the iri to a 
/// shared iri using a `prefix`) and a `local_name`.
/// 
/// An `IriNode` can be used as a [`Subject`], [`Predicate`], [`Object`] or 
/// [`Graph`]; this struct implements [`Into`] for all of these types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IriNode<'a> {
    pub(crate) namespace: Namespace<'a>,
    pub(crate) local_name: Cow<'a, str>
}

impl<'a> IriNode<'a> {
    /// Create a new `IriNode`.
    pub fn new<C: Into<Cow<'a, str>>>(
        namespace: Namespace<'a>, local_name: C
    ) -> IriNode<'a> {
        IriNode { namespace, local_name: local_name.into() }
    }

    /// Allows you to create a new `IriNode` which is composed of static values 
    /// known at compile time, exported via [`Predicate`].
    /// 
    /// This function is private in order to prevent users bypassing URL 
    /// validation on creation.
    pub(crate) const fn new_const(
        namespace: Namespace<'static>, local_name: &'static str
    ) -> IriNode<'static> {
        IriNode { namespace, local_name: Cow::Borrowed(local_name) }
    }

    /// Get a reference to this `IriNode`'s `local_name`.
    pub fn local_name(&'a self) -> &'a str {
        &self.local_name
    }

    /// Consume this `IriNode`, returning a tuple of its `namespace` and 
    /// `local_name`.
    pub fn into_parts(self) -> (Namespace<'a>, Cow<'a, str>) {
        (self.namespace, self.local_name)
    }

    /// Serves as a custom implementation of [`Into<StagingNode<'a>>`] for an 
    /// `IriNode`, which takes in a mutable reference to a [`NamespaceStore`] in 
    /// order to retrieve the [`NamespaceId`] for an interened `Namespace`.
    pub(crate) fn into_staging(
        self, store: &mut NamespaceStore
    ) -> StagingNode<'a> {
        let namespace_id = store.intern_namespace(self.namespace);

        StagingNode::Iri(StagingIriNode {
            namespace_id, local_name: self.local_name
        })
    }
}

impl<'a> Into<Graph<'a>> for IriNode<'a> {
    #[inline]
    fn into(self) -> Graph<'a> {
        Graph(self)
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
        Predicate(self)
    }
}

impl<'a> Into<Subject<'a>> for IriNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Iri(self)
    }
}

impl<'a> Into<Graph<'a>> for &'a IriNode<'a> {
    #[inline]
    fn into(self) -> Graph<'a> {
        Graph(self.clone())
    }
}

impl<'a> Into<Object<'a>> for &'a IriNode<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Iri(self.clone())
    }
}

impl<'a> Into<Predicate<'a>> for &'a IriNode<'a> {
    #[inline]
    fn into(self) -> Predicate<'a> {
        Predicate(self.clone())
    }
}

impl<'a> Into<Subject<'a>> for &'a IriNode<'a> {
    #[inline]
    fn into(self) -> Subject<'a> {
        Subject::Iri(self.clone())
    }
}

impl<'a> ToStatic for IriNode<'a> {
    type StaticType = IriNode<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        IriNode {
            namespace: self.namespace.to_static(),
            local_name: Cow::Owned(self.local_name.clone().into_owned())
        }
    }
}

/// An [`IriNode`] that stores an already interned [`Namespace`]'s `NamespaceId`.
/// 
/// This type still retains its lifetime, as it can still reference a temporary 
/// value prior to interning the node itself.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StagingIriNode<'a> {
    namespace_id: NamespaceId,
    local_name: Cow<'a, str>
}

impl<'a> StagingIriNode<'a> {
    /// Get the `namespace_id` for this `StagingIriNode`.
    pub(crate) fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    /// Get a reference to the `local_name` for this `StagingIriNode`.
    pub(crate) fn local_name(&self) -> &str {
        &self.local_name
    }
}

impl<'a> ToStatic for StagingIriNode<'a> {
    type StaticType = StagingIriNode<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        StagingIriNode {
            namespace_id: self.namespace_id(),
            local_name: Cow::Owned(self.local_name.clone().into_owned())
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