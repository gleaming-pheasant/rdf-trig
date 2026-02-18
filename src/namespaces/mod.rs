use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::ops::Deref;

use crate::FastIndexSet;
use crate::traits::WriteTriG;

pub mod statics;

#[derive(Debug)]
pub struct Namespace {
    prefix: Cow<'static, str>,
    iri: Cow<'static, str>
}

impl Namespace {
    /// Create a new [`Namespace`].
    /// 
    /// Prefer [`Namespace::new_const`] when declaring a `Namespace` with only 
    /// `static` prefix and iri values.
    pub fn new<P, I>(prefix: P, iri: I) -> Namespace
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>
    {
        Namespace {
            prefix: prefix.into(),
            iri: iri.into()
        }
    }

    /// Create a new [`Namespace`] from &'static str parts.
    pub const fn new_const(prefix: &'static str, iri: &'static str) -> Namespace {
        Namespace {
            prefix: Cow::Borrowed(prefix),
            iri: Cow::Borrowed(iri)
        }
    }

    /// Return a `(Cow<'static, str>, Cow<'static, str>)` containing this 
    /// `Namespace`'s `prefix` and `iri`.
    pub fn into_parts(self) -> (Cow<'static, str>, Cow<'static, str>) {
        (self.prefix, self.iri)
    }

    /// Return a reference to this `Namespace`'s `prefix`.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Return a reference to this `Namespace`'s `iri`.
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct NamespaceId(u32);

impl NamespaceId {
    pub(crate) fn from(ix: usize) -> NamespaceId {
        debug_assert!(ix <= u32::MAX as usize);
        NamespaceId(ix as u32)
    }
}

impl Deref for NamespaceId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub(crate) struct NamespaceStore {
    store: FastIndexSet<Namespace>
}

impl NamespaceStore {
    /// Create a new [`NamespaceStore`].
    pub(crate) fn new() -> NamespaceStore {
        NamespaceStore {
            store: FastIndexSet::default()
        }
    }

    /// Add a [`Namespace`] to this `NamespaceStore`, returning its index 
    /// [`NamespaceId`].
    pub(crate) fn intern_namespace(&mut self, ns: Namespace) -> NamespaceId {
        NamespaceId::from(self.store.insert_full(ns).0)
    }

    /// Retrieve a reference to a `Namespace` from this `NamespaceStore` using 
    /// the provided `NamespaceId`.
    pub(crate) fn query_namespace(&self, ns_id: NamespaceId) -> &Namespace {
        &self.store.get_index(*ns_id as usize).unwrap()
    }
}

impl WriteTriG for NamespaceStore {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for ns in &self.store {
            writer.write_all(b"@prefix ");
            writer.write_all(ns.prefix().as_bytes());
            writer.write_all(b": <");
            writer.write_all(ns.iri().as_bytes());
            writer.write_all(b"> .\n");
        }

        Ok(())
    }
}