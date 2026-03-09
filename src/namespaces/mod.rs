use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::ops::Deref;

use url::Url;

use crate::FastIndexSet;
use crate::errors::RdfTrigError;
use crate::traits::WriteTriG;
use crate::utils::{write_escaped_local_name, write_escaped_url_component};

pub mod predicates;
pub mod statics;

/// A `Namespace` is a mapping between a `prefix` and an `iri`.
/// 
/// See [`crate`] documentation for details on this crates relationship with 
/// IRIs.
#[derive(Debug)]
pub struct Namespace<'a> {
    prefix: Cow<'a, str>,
    iri: Cow<'a, str>
}

impl<'a> Namespace<'a> {
    /// Create a new [`Namespace`].
    /// 
    /// Returns a `RdfTrigError::InvalidIri` if the `iri` cannot be parsed as a 
    /// url.
    pub fn new<P, I>(prefix: P, iri: I) -> Result<Namespace<'a>, RdfTrigError<'a>>
    where
        P: Into<Cow<'a, str>>,
        I: Into<Cow<'a, str>>
    {
        let iri = iri.into();
        
        // Guard clause to prevent invalid IRIs.
        if Url::parse(&iri).is_err() {
            return Err(RdfTrigError::InvalidIri(iri));
        }

        Ok(Namespace {
            prefix: prefix.into(),
            iri
        })
    }

    /// Create a new [`Namespace`] from &'static str parts.
    /// 
    /// This is a private function as it does not perform validation on the 
    /// `iri`.
    pub(crate) const fn new_const(
        prefix: &'static str, iri: &'static str
    ) -> Namespace<'a> {
        Namespace {
            prefix: Cow::Borrowed(prefix),
            iri: Cow::Borrowed(iri)
        }
    }

    /// Return a `(Cow<'static, str>, Cow<'static, str>)` containing this 
    /// `Namespace`'s `prefix` and `iri`.
    pub fn into_parts(self) -> (Cow<'a, str>, Cow<'a, str>) {
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
    pub(crate) fn set_prefix(&mut self, prefix: Cow<'a, str>) -> () {
        self.prefix = prefix;
    }
}

impl<'a> Hash for Namespace<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iri.hash(state);
    }
}

impl<'a> PartialEq for Namespace<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iri == other.iri
    }

    fn ne(&self, other: &Self) -> bool {
        self.iri != other.iri
    }
}

impl<'a> Eq for Namespace<'a> {}

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
pub(crate) struct NamespaceStore<'a> {
    store: FastIndexSet<Namespace<'a>>
}

impl<'a> NamespaceStore<'a> {
    /// Create a new [`NamespaceStore`].
    pub(crate) fn new() -> NamespaceStore<'a> {
        NamespaceStore {
            store: FastIndexSet::default()
        }
    }

    /// Add a [`Namespace`] to this `NamespaceStore`, returning its index 
    /// [`NamespaceId`].
    pub(crate) fn intern_namespace(&mut self, ns: Namespace<'a>) -> NamespaceId {
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
    pub(crate) fn query_namespace(&self, ns_id: NamespaceId) -> &Namespace<'a> {
        self.store.get_index(*ns_id as usize).unwrap()
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
    ) -> Option<&Namespace<'a>> {
        self.store.iter().find(|this_ns| {
            this_ns.prefix() == ns.prefix()
        })
    }

    /// Attempt to append 
    /// 
    /// __Panics!__ If the number of matching `prefix`es is greater than 255. 
    /// Something has gone seriously wrong if you've got 255 matching 
    /// namespace prefixes!
    pub(crate) fn find_new_prefix(
        &self, mut ns: Namespace<'a>
    ) -> Namespace<'a> {
        let mut suffix: u8 = 0;

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

impl<'a> WriteTriG for NamespaceStore<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for ns in &self.store {
            writer.write_all(b"@prefix ")?;
            write_escaped_local_name(writer, ns.prefix())?;
            writer.write_all(b": <")?;
            write_escaped_url_component(writer, ns.iri())?;
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
            Namespace::new("test", "http://example1.com").unwrap()
        );

        // Same prefix, different iri, should append to the prefix.
        let ns_two = store.intern_namespace(
            Namespace::new("test", "http://example2.com").unwrap()
        );

        // Same prefix, different iri again.
        let ns_three = store.intern_namespace(
            Namespace::new("test", "http://example3.com").unwrap()
        );

        // Exactly the same as ns_two.
        let ns_four = store.intern_namespace(
            Namespace::new("test", "http://example2.com").unwrap()
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
            Namespace::new("one", "http://examples.com/").unwrap()
        );

        let ns_two = store.intern_namespace(
            Namespace::new("two", "http://examples.com/").unwrap()
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