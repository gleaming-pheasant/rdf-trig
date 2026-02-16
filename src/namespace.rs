use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// A [`Namespace`] consists of a `prefix` and an `iri`.
/// 
/// The `prefix` is the short form name that will precede any `iri` endpoint 
/// in textual representations of a [`Node::Iri`].
///
/// When a [`Namespace`] is interned, the `prefix` in any outputs may differ 
/// from the defined `prefix`. This will occur if the same [`Namespace`] is 
/// interned with a different `prefix` elsewhere, and occurs because a 
/// [`Namespace`] only implements [`Hash`] and [`PartialEq`] on its `iri`.
#[derive(Debug)]
pub struct Namespace {
    prefix: Cow<'static, str>,
    iri: Cow<'static, str>
}

impl Namespace {
    /// Create a new [`Namespace`].
    pub fn new<C: Into<Cow<'static, str>>>(prefix: C, iri: C) -> Namespace {
        Namespace {
            prefix: prefix.into(),
            iri: iri.into()
        }
    }

    /// Get a reference to the `prefix`.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get a reference to the `iri`.
    pub fn iri(&self) -> &str {
        &self.iri
    }
}

impl Hash for Namespace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iri.hash(state);
    }
}

impl PartialEq for Namespace {
    fn eq(&self, other: &Self) -> bool {
        self.iri == other.iri
    }

    fn ne(&self, other: &Self) -> bool {
        self.iri != other.iri
    }
}

impl Eq for Namespace {}


/// A [`NamespaceId`] is a wrapper around the [`usize`] index for an interned 
/// [`Namespace`] in a collection (typically within a [`Store`]).
/// 
/// These cannot be constructed individually, to prevent generic [`usize`] 
/// values from being erroneously used.
#[derive(Debug, Eq, PartialEq)]
pub struct NamespaceId(usize);

impl NamespaceId {
    /// Create a new [`NamespaceId`], which should be the value returned as the 
    /// index wherever a [`Namespace`] is interned in a collection.
    /// 
    /// This function is the only way to create a [`NamespaceId`].
    /// 
    /// It is protected (crate use only), to prevent erroneous querying of an 
    /// interning collection where a user could query with a randomly derived 
    /// [`usize`].
    pub(crate) fn new(id: usize) -> NamespaceId {
        NamespaceId(id)
    }
}

impl Deref for NamespaceId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}