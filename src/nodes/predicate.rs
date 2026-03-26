use crate::nodes::NamedNode;
use crate::traits::ToStatic;

/// A `Predicate` forms the middle part of any `Triple`, establishing the 
/// relationship between a `Subject` and an `Object`.
/// 
/// A `Predicate` can only be a `NamedNode`, therefore, it can only be 
/// constructed using [`Into<Predicate>`] from a `NamedNode`.
/// 
/// Because many `Predicate`s are frequently reused, many `const` `NamedNode`s 
/// are exported in the [`crate::nodes::named::properties`] module.
/// 
/// Without being added to a [`Triple`](crate::triples::Triple) and stored in a 
/// [`TripleStore`](crate::triplestore::TripleStore), this struct serves no 
/// practical purpose.
#[derive(Clone, Debug)]
pub struct Predicate<'a>(pub(crate) NamedNode<'a>);

impl<'a> Predicate<'a> {
    /// Create a new `Predicate` from 'static values. Only accessible within 
    /// this crate to bypass IRI validation.
    pub(crate) const fn new_const(iri: &'static str) -> Predicate<'a> {
        Predicate(NamedNode::new_const(iri))
    }
}

impl<'a> From<&Predicate<'a>> for Predicate<'a> {
    fn from(p: &Predicate<'a>) -> Self {
        p.clone()
    }
}

impl<'a> ToStatic for Predicate<'a> {
    type StaticType = Predicate<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        Predicate(self.0.to_static())
    }
}