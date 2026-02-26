use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::ops::Deref;

use crate::FastIndexSet;
use crate::traits::WriteTriG;

pub mod statics;

/// A `Namespace` is a mapping between a `prefix` and an `iri`.
/// 
/// Following [Turtle](https://www.w3.org/TR/2014/REC-turtle-20140225/#sec-escapes) 
/// and [TriG](https://www.w3.org/TR/trig/#sec-escapes) formatting rules, this 
/// crate does not escape iris with "%-encoding", but does accept "%-encoded" 
/// iris. Using W3C's example, both `http://a.example/foo-bar` and 
/// `http://a.example/%66oo-bar` are accepted, but they are not equivalent.
/// 
/// When `Namespace`s are stored, they are interned to prevent excessive 
/// duplication of long strings. This crate only implements prefixes for output, 
/// so any endpoints added to an interned `Namespace` are simply appended to the 
/// iri.
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

    /// This function allow updating in-place this `Namespace`'s `prefix`.
    /// 
    /// This should be used internally only to update the `prefix` where a clash 
    /// has been identified whilst interning a `Namespace`.
    pub(crate) fn set_prefix(&mut self, prefix: Cow<'static, str>) -> () {
        self.prefix = prefix;
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

/// A `NamespaceStore` is a wrapper around a series of collections for interning 
/// `Namespace`s.
/// 
/// It will only allow an IRI to be stored once, and checks for the presence of 
/// trailing slashes (if a trailing `/` or `#` is missing, )
/// 
/// This crate allows users to define prefixes for namespaces to prevent 
/// overloading the "namespace store" of any receiving databases (such as 
/// [GraphDB](https://graphdb.ontotext.com/)), but it also needs a way to avoid 
/// prefix collisions.
/// 
/// Therefore, the `NamespaceStore` provides a means to check for prefix 
/// collisions on insert, but retains 
/// 
/// If a `prefix` collision is detected, the `NamespaceStore` will automatically 
/// append an incrementing number to the end of the prefix.
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
        // Namespace implements `Hash` only on the `iri` field, so this works.
        match self.store.get_index_of(&ns) {
            // Already exists, so end of.
            Some(ix) => NamespaceId::from(ix),
            // Does not exist, but does the prefix?
            None => {
                match self.query_existing_prefix(&ns) {
                    // Yes, prefix exists, add to it.
                    Some(_) => {
                        NamespaceId::from(self.store.insert_full(
                            self.find_new_prefix(ns)
                        ).0)
                    },
                    // No, prefix doesn't exist, so add and be done.
                    None => NamespaceId::from(self.store.insert_full(ns).0)
                }
            }
        }
    }

    /// Retrieve a reference to a `Namespace` from this `NamespaceStore` using 
    /// the provided `NamespaceId`.
    pub(crate) fn query_namespace(&self, ns_id: NamespaceId) -> &Namespace {
        &self.store.get_index(*ns_id as usize).unwrap()
    }

    /// Query the `NamespaceStore` for existing `Namespace`s with the same 
    /// `prefix` as the provided `Namespace`.
    /// 
    /// This is used to ensure that a `prefix` isn't being used for two distinct 
    /// `iri`s, and should only be called after you've verified a `Namespace` 
    /// with the same `iri` hasn't already been interned.
    /// 
    /// This is a costly *O(n)* operation, and relies on the fact that 
    /// namespaces/prefixes are typically small in number.
    pub(crate) fn query_existing_prefix(
        &self, ns: &Namespace
    ) -> Option<&Namespace> {
        self.store.iter().find(|this_ns| {
            this_ns.prefix() == ns.prefix()
        })
    }

    /// Attempt to append 
    /// 
    /// __Panics!__ If the number of matching `prefix`es is greater than 65,535. 
    /// Something has gone seriously wrong if you've got 65,535 matching 
    /// namespace prefixes!
    pub(crate) fn find_new_prefix(
        &self, mut ns: Namespace
    ) -> Namespace {
        let mut suffix: u16 = 0;

        let prefix_base: String = ns.prefix().to_string();

        loop {
            ns.set_prefix(format!("{prefix_base}{suffix}").into());
            
            if self.query_existing_prefix(&ns).is_none() {
                // Prefix with this suffix doesn't exist!
                break ns;
            }

            suffix += 1;
        }
    }
}

impl WriteTriG for NamespaceStore {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for ns in &self.store {
            writer.write_all(b"@prefix ")?;
            writer.write_all(ns.prefix().as_bytes())?;
            writer.write_all(b": <")?;
            writer.write_all(ns.iri().as_bytes())?;
            writer.write_all(b"> .\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that duplicate prefixes for different iris are appended to 
    /// correctly, and that they have the correct ownership types.
    #[test]
    fn test_prefix_appends() {
        let mut store = NamespaceStore::new();

        let ns_one= store.intern_namespace(
            Namespace::new("test", "http://example1.com")
        );

        // Same prefix, different iri, should append to the prefix.
        let ns_two = store.intern_namespace(
            Namespace::new("test", "http://example2.com")
        );

        // Same prefix, different iri again.
        let ns_three = store.intern_namespace(
            Namespace::new("test", "http://example3.com")
        );

        // Exactly the same as ns_two.
        let ns_four = store.intern_namespace(
            Namespace::new("test", "http://example2.com")
        );

        assert!(
            matches!(store.query_namespace(ns_one).prefix, Cow::Borrowed(_))
        );
        assert!(
            matches!(store.query_namespace(ns_two).prefix, Cow::Owned(_))
        );

        assert_eq!(store.query_namespace(ns_two).prefix(), "test0");
        assert_eq!(store.query_namespace(ns_three).prefix(), "test1");
        assert_eq!(store.query_namespace(ns_four).prefix(), "test0");
        assert_eq!(
            store.query_namespace(ns_two).prefix(),
            store.query_namespace(ns_four).prefix()
        );
    }

    /// Test that if the same IRI is used with different prefixes, only the 
    /// first prefix declared is used.
    #[test]
    fn test_same_iri_different_prefix() {
        let mut store = NamespaceStore::new();

        let ns_one = store.intern_namespace(
            Namespace::new("one", "http://examples.com/")
        );

        let ns_two = store.intern_namespace(
            Namespace::new("two", "http://examples.com/")
        );

        assert_eq!(
            store.query_namespace(ns_one).prefix(),
            store.query_namespace(ns_two).prefix()
        );

        assert_eq!(store.query_namespace(ns_two).prefix(), "one");

        assert!(
            matches!(store.query_namespace(ns_one).prefix, Cow::Borrowed(_))
        );
        assert!(
            matches!(store.query_namespace(ns_two).prefix, Cow::Borrowed(_))
        );
    }
}