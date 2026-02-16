use std::borrow::Cow;

use crate::literal::Literal;
use crate::namespace::Namespace;

/// A [`RawIri`] is an [`Iri`] which contains a full [`Namespace`].
/// 
/// This struct provides a staging ground for declaring a [`Node`] before the 
/// [`Namespace`] itself has been interned in a [`TripleStore`], or before the 
/// interned [`NamespaceId`] is known.
#[derive(Debug)]
pub struct RawIri {
    namespace: Namespace,
    endpoint: Cow<'static, str>
}

impl RawIri {
    /// Create a new [`RawIri`].
    pub fn new<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> RawIri {
        RawIri {
            namespace,
            endpoint: endpoint.into()
        }
    }
}

impl Into<RawNode> for RawIri {
    fn into(self) -> RawNode {
        RawNode::RawIri(self)
    }
}


/// An enumerator of RDF nodes, which can be either a [`RawIri`] or a [`Literal`].
/// 
/// [`Literal`] and [`RawIri`] both implement [`Into<RawNode>`] for ease parsing.
#[derive(Debug)]
pub enum RawNode {
    RawIri(RawIri),
    Literal(Literal)
}

/// A [`RawTriple`] serves as the staging ground for build [`Triple`]s, which 
/// takes "Raw" implementations of RDF types before any contained IRIs can be 
/// interned.
/// 
/// Because this crate does not implement "Blank Nodes", `subject` and 
/// `predicate` can only be [`RawIri`]s, while the `object` is a [`RawNode`] 
/// (either a [`Literal`] or a [`RawIri`]).
#[derive(Debug)]
pub struct RawTriple {
    subject: RawIri,
    predicate: RawIri,
    object: RawNode
}

impl RawTriple {
    /// Create a new [`RawTriple`].
    pub fn new<N: Into<RawNode>>(subject: RawIri, predicate: RawIri, object: N) -> RawTriple {
        RawTriple {
            subject,
            predicate,
            object: object.into()
        }
    }
}