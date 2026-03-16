use std::io::{self, Write};
use std::ops::Deref;

use crate::FastIndexSet;
use crate::namespaces::Namespace;
use crate::traits::{ToStatic, WriteTriG};
use crate::utils::{write_escaped_local_name, write_escaped_url_component};

/// A `NamespaceId` is a wrapper around a `u32` and is only retrievable by 
/// converting the `usize` index from an [`IndexSet`](indexmap::IndexSet) (or a 
/// [`FastIndexSet`] for the purposes of this crate).
/// 
/// This will cause the application to panic if the number of interned nodes 
/// exceeds [`u32::MAX`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct NamespaceId(pub(crate) u32);

impl NamespaceId {
    /// Create a new `NodeId` by casting the provided `usize` to a `u32`.
    /// 
    /// Panics if `ix` is greater than [`u32::MAX`].
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

/// A wrapper around a [`FastIndexSet<Namespace<'static>`] which serves to store 
/// unique "nodes" and hand out [`NamespaceId`]s as references to the interned 
/// [`Namespace`]s.
/// 
/// It will only allow an IRI to be stored once.
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
pub(crate) struct NamespaceStore(FastIndexSet<Namespace<'static>>);

impl NamespaceStore {
    /// Create a new `NamespaceStore`.
    pub(crate) fn new() -> NamespaceStore {
        NamespaceStore(FastIndexSet::default())
    }

    /// Add a [`Namespace`] to this `NamespaceStore`, returning its index 
    /// [`NamespaceId`].
    pub(crate) fn intern_namespace<'a, N>(&mut self, ns: N) -> NamespaceId
    where
        N: Into<Namespace<'a>>
    {
        let ns_ref = ns.into();

        // Namespace implements `Hash` only on the `iri` field, so this works.
        match self.0.get_index_of(&ns_ref) {
            // Already exists, so return existing, using existing prefix.
            Some(ix) => NamespaceId::from(ix),
            // Does not exist, but does the prefix?
            None => {
                match self.query_existing_prefix(&ns_ref) {
                    // Yes, prefix exists, add to it.
                    Some(_) => NamespaceId::from(
                        self.0.insert_full(
                            self.find_new_prefix(ns_ref).to_static()
                        ).0
                    ),
                    // No, prefix doesn't exist
                    None => NamespaceId::from(
                        self.0.insert_full(ns_ref.to_static()).0
                    )
                }
            }
        }
    }

    /// Retrieve a reference to a `Namespace` from this `NamespaceStore` using 
    /// the provided `NamespaceId`.
    pub(crate) fn query_namespace(
        &self, namespace_id: NamespaceId
    ) -> &Namespace<'static> {
        self.0.get_index(*namespace_id as usize).unwrap()
    }

    /// Query the `NamespaceStore` for existing `Namespace`s with the same 
    /// `prefix` as the provided `Namespace`.
    /// 
    /// This is used to ensure that a `prefix` isn't being used for two distinct 
    /// `iri`s, and should only be called after you've verified a `Namespace` 
    /// with the same `iri` hasn't already been interned.
    /// 
    /// This is a costly *O(n)* operation, and relies on the assumption that 
    /// namespaces/prefixes are typically small in number.
    pub(crate) fn query_existing_prefix(
        &self, ns: &Namespace
    ) -> Option<&Namespace<'static>> {
        self.0.iter().find(|this_ns| {
            this_ns.prefix() == ns.prefix()
        })
    }

    /// Attempt to append 
    /// 
    /// __Panics!__ If the number of matching `prefix`es is greater than 255. 
    /// Something has gone seriously wrong if you've got 255 matching 
    /// namespace prefixes!
    pub(crate) fn find_new_prefix<'a>(
        &self, mut ns: Namespace<'a>
    ) -> Namespace<'a> {
        let mut suffix: u8 = 0;

        let prefix_base: String = ns.prefix().to_string();

        loop {
            ns.set_prefix(format!("{prefix_base}{suffix}").into());
            
            if self.query_existing_prefix(&ns).is_none() {
                // Prefix with this suffix doesn't exist!
                break ns
            }

            suffix += 1;
        }
    }
}

impl<'a> WriteTriG for NamespaceStore {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for ns in &self.0 {
            writer.write_all(b"@prefix ")?;
            write_escaped_local_name(writer, ns.prefix())?;
            writer.write_all(b": <")?;
            write_escaped_url_component(writer, ns.iri())?;
            writer.write_all(b"> .\n")?;
        }

        Ok(())
    }
}