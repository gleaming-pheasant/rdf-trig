use std::io::{self, Write};

use crate::nodes::{NodeId, NodeStore};
use crate::triples::InternedTriple;
use crate::traits::WriteTriG;
#[cfg(feature = "tokio")]
use crate::traits::WriteTriGAsync;

/// A `TriGStore` is a wrapper around offsets to different levels of Node 
/// for representing data in a `TripleStore` in TriG.
/// 
/// It is built by simultaneously mapping `NodeId`s to new `Vec`s of their type 
/// (graph, subject, predicate and object) and separate offsets which associate 
/// a higher-level type to their lower level type (graph-to-subject, 
/// subject-to-predicate, etc.). Each offset, therefore, will always be relative 
/// to the `NodeId` for a higher level (`subject_offsets` are how far each new 
/// subject appears relative to the same index in the `graph_nodes` `Vec`, etc.).
/// 
/// See: [data-oriented design](https://en.wikipedia.org/wiki/Data-oriented_design).
#[derive(Debug)]
pub(super) struct TriGStore {
    graph_nodes: Vec<Option<NodeId>>, // All existing graph `NodeId`s collected.
    // The location of the subjects in relation to each graph_node.
    subject_offsets: Vec<usize>,
    subject_nodes: Vec<NodeId>,
    predicate_offsets: Vec<usize>,
    predicate_nodes: Vec<NodeId>,
    object_offsets: Vec<usize>,
    object_nodes: Vec<NodeId>
}

impl TriGStore {
    /// Create a new `TriGStore` by sorting and looping through the provided 
    /// [`Vec<&InternedTriple>`].
    /// 
    /// [`InternedTriple`] implements [`Ord`] via the [`derive`] macro, which 
    /// sorts the order by the order of the fields (Graph > Subject > Predicate 
    /// > Object).
    pub(super) fn new(mut triples: Vec<&InternedTriple>) -> TriGStore {
        // InternedTriple implements 
        triples.sort_unstable();

        let mut graph_nodes = vec![];
        let mut subject_offsets = vec![];
        let mut subject_nodes = vec![];
        let mut predicate_offsets = vec![];
        let mut predicate_nodes = vec![];
        let mut object_offsets = vec![];
        let mut object_nodes = Vec::with_capacity(triples.len());

        let mut prev_graph = None;
        let mut prev_subject = None;
        let mut prev_predicate = None;

        for triple in triples {
            if graph_nodes.is_empty() || Some(triple.graph()) != prev_graph {
                // If there is a graph, there must be a subject. This tells us 
                // that in the Vec<InternedTriple> this row (e.g. ix = 0) has a 
                // matching Subject.
                // We add a Subject for every iteration, so the current len must 
                // be where we're adding the new subject.
                subject_offsets.push(subject_nodes.len());
                graph_nodes.push(triple.graph());
                prev_graph = Some(triple.graph());
                prev_subject = None;
                prev_predicate = None;
            }

            if prev_subject.is_none() || Some(triple.subject()) != prev_subject {
                predicate_offsets.push(predicate_nodes.len());
                subject_nodes.push(triple.subject());
                prev_subject = Some(triple.subject());
                prev_predicate = None;
            }

            if prev_predicate.is_none() || Some(triple.predicate()) != prev_predicate {
                object_offsets.push(object_nodes.len());
                predicate_nodes.push(triple.predicate());
                prev_predicate = Some(triple.predicate());
            }

            object_nodes.push(triple.object());
        }

        subject_offsets.push(subject_nodes.len());
        predicate_offsets.push(predicate_nodes.len());
        object_offsets.push(object_nodes.len());

        TriGStore { graph_nodes,
            subject_offsets,
            subject_nodes,
            predicate_offsets,
            predicate_nodes,
            object_offsets,
            object_nodes
        }
    }

    /// A custom implementation of the `WriteTriG::write_trig()` function, which 
    /// takes in a reference to the `NodeStore` contained within the main 
    /// `TripleStore`.
    /// 
    /// This function exists in this module to separate the unique 
    /// implementation of the offsets in the `TriGStore` from what should 
    /// otherwise be basic string formatting.
    pub(super) fn write_store_trig<W: Write>(
        &self, writer: &mut W, node_store: &NodeStore
    ) -> io::Result<()> {
        // Might as well always write, even if no literals appear, causes no harm
        writer.write_all(b"@prefix xsd: <http://www.w3.org/2001/XMLSchema#> . ")?;
        
        // Loop one, all graphs (including None/default)
        for (graph_ix, &graph_id) in self.graph_nodes.iter().enumerate() {
            let subject_range = self.subject_offsets[graph_ix]..
                self.subject_offsets[graph_ix + 1];

            if let Some(id) = graph_id {
                let graph_node = node_store.query_node(id);
                graph_node.write_trig(writer)?;
                writer.write_all(b" { ")?;
            }

            // This is a new iteration for every graph, and so the offsets are 
            // always relative to the index of this graph. Think "array of 
            // structs" (yay, DoD!).
            for sub_offset in subject_range {
                let preds_start = self.predicate_offsets[sub_offset];
                let preds_end = self.predicate_offsets[sub_offset + 1];
                let pred_count = preds_end - preds_start;

                // Location ix of the actual current subject.
                let sub_id = self.subject_nodes[sub_offset];
                let subject_node = node_store.query_node(sub_id);

                subject_node.write_trig(writer)?;
                writer.write_all(b" ")?;

                for (pred_ix, pred_abs_ix) in (preds_start..preds_end).enumerate() {
                    let objs_start = self.object_offsets[pred_abs_ix];
                    let objs_end = self.object_offsets[pred_abs_ix + 1];
                    let objs_count = objs_end - objs_start;

                    let pred_id = self.predicate_nodes[pred_abs_ix];
                    let predicate_node = node_store.query_node(pred_id);

                    predicate_node.write_trig(writer)?;

                    writer.write_all(b" ")?;


                    for (obj_ix, obj_abs_ix) in (objs_start..objs_end).enumerate() {
                        let obj_id = self.object_nodes[obj_abs_ix];
                        let object_node = node_store.query_node(obj_id);

                        object_node.write_trig(writer)?;
                        
                        // Not finished with this predicate yet, so comma.
                        if obj_ix < objs_count - 1 {
                            writer.write_all(b" , ")?;
                        }
                    }
                    
                    if pred_ix < pred_count - 1 {
                        // Not finished with this subject yet, so semicolon.
                        writer.write_all(b" ; ")?;
                    } else {
                        writer.write_all(b" . ")?; // End of preds and objs.
                    }
                }
            }

            if graph_id.is_some() {
                writer.write_all(b"} ")?;
            }
        }

        Ok(())
    }
}