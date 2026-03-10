use std::borrow::Cow;

use crate::errors::RdfTrigError;
use crate::namespaces::Namespace;
use crate::nodes::object::Object;
use crate::nodes::predicate::Predicate;
use crate::nodes::subject::Subject;

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
    fn into(self) -> Object<'a> {
        Object::Iri(self)
    }
}

impl<'a> Into<Predicate<'a>> for IriNode<'a> {
    fn into(self) -> Predicate<'a> {
        Predicate::new(self)
    }
}

impl<'a> Into<Subject<'a>> for IriNode<'a> {
    fn into(self) -> Subject<'a> {
        Subject::Iri(self)
    }
}