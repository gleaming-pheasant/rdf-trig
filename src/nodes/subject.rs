use crate::nodes::{BlankNode, IriNode};

/// A `Subject` forms part of a `Triple`; it defines the node that the 
/// rest of the `Triple` expands upon.
/// 
/// A `Subject` can only be a `BlankNode` or and `IriNode` and so can only be 
/// constructed using the respective [`Into<Subject>`] implementations for these 
/// types.
#[derive(Debug)]
pub enum Subject<'a> {
    Blank(BlankNode<'a>),
    Iri(IriNode<'a>)
}