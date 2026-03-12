pub mod statics;
pub(crate) mod store;

use std::borrow::Cow;
use std::hash::{Hash, Hasher};

use url::Url;

use crate::errors::RdfTrigError;
use crate::traits::ToStatic;

/// A `Namespace` is a mapping between a `prefix` and an `iri`.
/// 
/// See [`crate`] documentation for details on this crates relationship with 
/// IRIs.
#[derive(Clone, Debug)]
pub struct Namespace<'a> {
    prefix: Cow<'a, str>,
    iri: Cow<'a, str>
}

impl<'a> Namespace<'a> {
    /// Create a new [`Namespace`].
    /// 
    /// Returns a `RdfTrigError::InvalidIri` if the `iri` cannot be parsed as a 
    /// url.
    pub fn new<P, I>(prefix: P, iri: I) -> Result<Namespace<'a>, RdfTrigError>
    where
        P: Into<Cow<'a, str>>,
        I: Into<Cow<'a, str>>
    {
        let iri = iri.into();
        
        // Guard clause to prevent invalid IRIs.
        if Url::parse(&iri).is_err() {
            return Err(RdfTrigError::InvalidIri(iri.to_string()));
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
    ) -> Namespace<'static> {
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
    pub(crate) fn set_prefix(&mut self, prefix: Cow<'static, str>) -> () {
        self.prefix = prefix;
    }
}

impl<'a> ToStatic for Namespace<'a> {
    type StaticType = Namespace<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        Namespace {
            prefix: Cow::Owned(self.prefix.clone().into_owned()),
            iri: Cow::Owned(self.iri.clone().into_owned())
        }
    }
}

impl<'a> From<&Namespace<'a>> for Namespace<'a> {
    fn from(n: &Namespace<'a>) -> Self {
        n.clone()
    }
}

impl Hash for Namespace<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iri.hash(state);
    }
}

impl PartialEq for Namespace<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.iri == other.iri
    }

    fn ne(&self, other: &Self) -> bool {
        self.iri != other.iri
    }
}

impl Eq for Namespace<'_> {}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     /// Test that duplicate prefixes for different iris are appended to 
//     /// correctly, and that they have the correct ownership types.
//     #[test]
//     fn test_prefix_appends() {
//         let mut store = NamespaceStore::new();

//         let ns_one= store.intern_namespace(
//             Namespace::new("test", "http://example1.com").unwrap()
//         );

//         // Same prefix, different iri, should append to the prefix.
//         let ns_two = store.intern_namespace(
//             Namespace::new("test", "http://example2.com").unwrap()
//         );

//         // Same prefix, different iri again.
//         let ns_three = store.intern_namespace(
//             Namespace::new("test", "http://example3.com").unwrap()
//         );

//         // Exactly the same as ns_two.
//         let ns_four = store.intern_namespace(
//             Namespace::new("test", "http://example2.com").unwrap()
//         );

//         assert!(
//             matches!(store.query_namespace(ns_one).prefix, Cow::Borrowed(_))
//         );
//         assert!(
//             matches!(store.query_namespace(ns_two).prefix, Cow::Owned(_))
//         );

//         assert_eq!(store.query_namespace(ns_two).prefix(), "test0");
//         assert_eq!(store.query_namespace(ns_three).prefix(), "test1");
//         assert_eq!(store.query_namespace(ns_four).prefix(), "test0");
//         assert_eq!(
//             store.query_namespace(ns_two).prefix(),
//             store.query_namespace(ns_four).prefix()
//         );
//     }

//     /// Test that if the same IRI is used with different prefixes, only the 
//     /// first prefix declared is used.
//     #[test]
//     fn test_same_iri_different_prefix() {
//         let mut store = NamespaceStore::new();

//         let ns_one = store.intern_namespace(
//             Namespace::new("one", "http://examples.com/").unwrap()
//         );

//         let ns_two = store.intern_namespace(
//             Namespace::new("two", "http://examples.com/").unwrap()
//         );

//         assert_eq!(
//             store.query_namespace(ns_one).prefix(),
//             store.query_namespace(ns_two).prefix()
//         );

//         assert_eq!(store.query_namespace(ns_two).prefix(), "one");

//         assert!(
//             matches!(store.query_namespace(ns_one).prefix, Cow::Borrowed(_))
//         );
//         assert!(
//             matches!(store.query_namespace(ns_two).prefix, Cow::Borrowed(_))
//         );
//     }
// }