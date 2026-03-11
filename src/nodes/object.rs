use crate::nodes::{BlankNode, IriNode};
use crate::nodes::literals::LiteralNode;

/// An `Object` is the final part of any `Triple`, effectively providing the 
/// value of a `Predicate` for a `Subject`.
/// 
/// An `Object` can be any of a `BlankNode`, `IriNode` or a literal node, and 
/// can be constructed using the [`Into<Object>`] implementations of any of 
/// those types.
/// 
/// Without being added to a [`Triple`](crate::triples::Triple) and stored in a 
/// [`DataStore`](crate::datastore::DataStore), this struct serves no practical 
/// purpose.
#[derive(Debug)]
pub enum Object<'a> {
    Blank(BlankNode<'a>),
    Iri(IriNode<'a>),
    Literal(LiteralNode<'a>)
}