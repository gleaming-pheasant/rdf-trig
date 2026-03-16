//! This module contains a collection of statically-declared, commonly used 
//! [`Namespace`]s.
//! 
//! Use the `const` function [`Namespace::new_const`] to create `const` 
//! `Namespace`s for use in your own applications.
use crate::nodes::Predicate;

use super::Namespace;

/// The [AO-Cat](https://doi.org/10.5281/zenodo.7818375) ontology.
pub const AOCAT: Namespace = Namespace::new_const(
    "aocat", "https://www.ariadne-infrastructure.eu/resource/ao/cat/1.1/"
);

/// A namespace for graphs forming part of the 
/// [ARIADNE portal](https://portal.ariadne-infrastructure.eu/).
pub const ARIADNEAPI: Namespace = Namespace::new_const(
    "ariadneapi", "https://ariadne-infrastructure.eu/"
);

/// A namespace for [ARIADNE](https://portal.ariadne-infrastructure.eu/) resources.
pub const ARIADNEPLUS: Namespace = Namespace::new_const(
    "ariadneplus", "https://ariadne-infrastructure.eu/aocat/"
);

/// The [CIDOC CRM](https://cidoc-crm.org/) ontology.
pub const CRM: Namespace = Namespace::new_const(
    "crm", "http://www.cidoc-crm.org/cidoc-crm/"
);

/// The [Dublin Core](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) ontology.
pub const DC: Namespace = Namespace::new_const(
    "dc", "http://purl.org/dc/elements/1.1/"
);

/// Dublin Core's [terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/#section-2) schema.
pub const DCTERMS: Namespace = Namespace::new_const(
    "dcterms", "http://purl.org/dc/terms/"
);

/// Terms used by [SPARQL 1.1](https://www.w3.org/ns/sparql#).
pub const SPARQL: Namespace = Namespace::new_const(
    "sparql", "http://www.w3.org/ns/sparql#"
);

/// The [Friend-of-a-Friend](https://xmlns.com/foaf/spec/) ontology.
pub const FOAF: Namespace = Namespace::new_const(
    "foaf", "http://xmlns.com/foaf/0.1/"
);

pub const LEXVO: Namespace = Namespace::new_const(
    "lexvo", "http://lexvo.org/ontology#"
);

pub const OWL: Namespace = Namespace::new_const(
    "owl", "http://www.w3.org/2002/07/owl#"
);

pub const RDF: Namespace = Namespace::new_const(
    "rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"
);

pub const RDF_TYPE: Predicate = Predicate::new_const(RDF, "type");

pub const RDFS: Namespace = Namespace::new_const(
    "rdfs", "http://www.w3.org/2000/01/rdf-schema#"
);

pub const SKOS: Namespace = Namespace::new_const(
    "skos", "http://www.w3.org/2004/02/skos/core#"
);

/// The XML Schema. Defines data types like dateTime and gYear.
pub const XSD: Namespace = Namespace::new_const(
    "xsd", "http://www.w3.org/2001/XMLSchema#"
);