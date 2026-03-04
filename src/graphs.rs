use std::borrow::Cow;
use std::io::{Result as IoResult, Write};
use std::ops::Deref;

use crate::FastIndexSet;
use crate::errors::RdfTrigError;
use crate::groups::triples::TripleView;
use crate::namespaces::{Namespace, NamespaceId};
use crate::traits::WriteTriG;
use crate::utils::{write_escaped_local_name, write_escaped_url_component};

/// A wrapper around an [`IndexSet<InternedGraph>`] which acts as a fast store 
/// for unique [`Graph`] values.
pub(crate) struct GraphStore {
    store: FastIndexSet<InternedGraph>
}

impl GraphStore {
    /// Create a new [`GraphStore`].
    pub(crate) fn new() -> GraphStore {
        GraphStore {
            store: FastIndexSet::default()
        }
    }

    /// Add an [`InternedGraph`] to this `GraphStore`.
    pub(crate) fn intern_graph(&mut self, graph: InternedGraph) -> GraphId {
        GraphId::from(self.store.insert_full(graph).0)
    }

    /// Get an `InternedGraph`'s `NamespaceId` by searching a provided `GraphId`.
    pub(crate) fn query_namespace(&self, graph_id: GraphId) -> NamespaceId {
        self.store
            .get_index(*graph_id as usize)
            .unwrap()
            .namespace_id()
    }

    /// Get an `InternedGraph`'s `NamespaceId` by searching a provided `GraphId`.
    pub(crate) fn query_endpoint(&self, graph_id: GraphId) -> &str {
        self.store
            .get_index(*graph_id as usize)
            .unwrap()
            .endpoint()
    }
}

impl Deref for GraphStore {
    type Target = FastIndexSet<InternedGraph>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<'a> IntoIterator for &'a GraphStore {
    type Item = &'a InternedGraph;
    type IntoIter = indexmap::set::Iter<'a, InternedGraph>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.store.iter()
    }
}


/// An [`InternedGraph`] is a [`Graph`] with its [`Namespace`] already 
/// registered with a [`NamespaceStore`](crate::namespaces::NamespaceStore).
/// 
/// It takes just the `endpoint` and the registered `Namespace`'s 
/// [`NamespaceId`].
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedGraph {
    namespace_id: NamespaceId,
    endpoint: Cow<'static, str>
}

impl InternedGraph {
    /// Create a new [`InternedGraph`].
    pub(crate) fn new(
        ns_id: NamespaceId, endpoint: Cow<'static, str>
    ) -> InternedGraph {
        InternedGraph { namespace_id: ns_id, endpoint: endpoint }
    }

    /// Get this `InternedGraph`'s `NamespaceId`.
    pub(crate) fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    /// Get a reference to this `InternedGraph`'s `endpoint`.
    pub(crate) fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// As with IRI nodes underlying `Subject`s, `Predicate`s and `Object`s, this 
/// is a combination of a [`Namespace`] and an `endpoint`.
/// 
/// See [`crate`] documentation for details on this crates relationship with 
/// IRIs.
#[derive(Debug, Hash)]
pub struct Graph {
    namespace: Namespace,
    endpoint: Cow<'static, str>
}

impl Graph {
    /// Create a new [`Graph`].
    pub fn new<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> Graph {
        Graph { namespace, endpoint: endpoint.into() }
    }

    /// Create a new [`Graph`], simultaneously declaring its [`Namespace`] from 
    /// a `prefix` and `iri`.
    /// 
    /// Returns a [`RdfTrigError::InvalidIri`] if the `iri` for the `Namespace` 
    /// is invalid.
    pub fn new_with_new_namespace<P, I, E>(
        prefix: P, iri: I, endpoint: E
    ) -> Result<Graph, RdfTrigError>
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
        E: Into<Cow<'static, str>>
    {
        Ok(Graph {
            namespace: Namespace::new(prefix, iri)?,
            endpoint: endpoint.into()
        })
    }

    /// Return a ([`Namespace`], [`Cow<'static, str>`]) tuple containing this 
    /// `Graph`'s `Namespace` and `endpoint`.
    pub fn into_parts(self) -> (Namespace, Cow<'static, str>) {
        (self.namespace, self.endpoint)
    }
}

/// A [`GraphId`] is a handle to a stored graph.
/// 
/// Once you declare a [`Graph`] and add it to a 
/// [`TripleStore`](crate::store::TripleStore), the returned `GraphId` can be 
/// used to assign further triples to the `Graph`.
/// 
/// This provides a low-cost way to build `Graph`s dynamically.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GraphId(u32);

impl GraphId {
    pub(crate) fn from(ix: usize) -> GraphId {
        debug_assert!(ix <= u32::MAX as usize);
        GraphId(ix as u32)
    }
}

impl Deref for GraphId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/// A `GraphView`, like other `...Graph` objects in this crate, provides 
/// references to interned data.
/// 
/// The `GraphView` in particular returns a reference to an interned 
/// [`Namespace`] and the endpoint of this `GraphView`'s underlying [`Graph`].
/// 
/// A `GraphView` cannot be constructed directly, and must be retrieved from a 
/// [`DataStore`](crate::store::DataStore).
#[derive(Debug)]
pub struct GraphView<'a> {
    namespace: &'a Namespace,
    endpoint: &'a str
}

impl<'a> GraphView<'a> {
    /// Private helper function to build a `GraphView` from references to a 
    /// [`Namespace`] and an `endpoint`.
    pub(crate) fn new(
        namespace: &'a Namespace, endpoint: &'a str
    ) -> GraphView<'a> {
        GraphView { namespace, endpoint }
    }

    /// Get a reference to the [`Namespace`] for this `GraphView`.
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    /// Get a reference to the `endpoint` [str] for this `GraphView`.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// A `FullGraphView`, like other `...View` objects, is a collection of 
/// references to interned data.
/// 
/// While interning [`Graph`]s in a [`DataStore`](crate::store::DataStore) is 
/// done by creating [`Quad`](crate::groups::quads::Quad)s using a generated 
/// [`GraphId`], a `FullGraphView` collects references to all 
/// [`Triple`](crate::groups::triples::Triple)s in the form of a [`TripleView`].
/// 
/// This is done to streamline writing of a graph in the TriG format; it 
/// prevents the need to intern or declare a full graph's IRI for every 
/// contained triple. See [`QuadView`](crate::groups::quads::QuadView) as a 
/// means to represent a `Graph` per `Triple` for other formats (e.g. N-Triples).
/// 
/// A `FullGraphView` cannot be constructed directly. It must be retrieved from 
/// a `DataStore`.
#[derive(Debug)]
pub struct FullGraphView<'a> {
    graph: GraphView<'a>,
    triples: Vec<TripleView<'a>>
}

impl<'a> FullGraphView<'a> {
    /// Private helper function to create a `FullGraphView` from the provided 
    /// [`Graph`] reference. Initialises an empty [`Vec`] for storing 
    /// [`TripleView`]s.
    pub(crate) fn new(graph: GraphView<'a>) -> FullGraphView<'a> {
        FullGraphView {
            graph,
            triples: Vec::new()
        }
    }

    /// Append the provided [`TripleView`] to the `triples` associated with this 
    /// `FullGraphView`.
    pub(crate) fn add_triple_view(&mut self, tv: TripleView<'a>) -> () {
        self.triples.push(tv);
    }
}

impl<'a> WriteTriG for FullGraphView<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        write_escaped_local_name(writer, self.graph.namespace().prefix())?;
        writer.write_all(b":")?;
        write_escaped_url_component(writer, self.graph.endpoint())?;
        writer.write_all(b" { ")?;

        for triple in &self.triples {
            triple.write_trig(writer)?;
            writer.write_all(b"\n")?;
        }

        writer.write_all(b"} \n")
    }
}