//! A collection of static [`Predicate`]s for widely used predicates for 
//! existing static [`Namespace`](crate::namespaces::Namespace)s.
use crate::namespaces::statics::{AOCAT, RDF, RDFS, SKOS};
use crate::nodes::Predicate;

/// aocat:has_ARIADNE_subject
pub const AOCAT_ARIADNE_SUBJECT: Predicate = Predicate::new_const(
    AOCAT, "has_ARIADNE_subject"
);

/// aocat:has_access_rights
pub const AOCAT_ACCESS_RIGHTS: Predicate = Predicate::new_const(
    AOCAT, "has_access_rights"
);

/// aocat:has_contributor
pub const AOCAT_CONTRIBUTOR: Predicate = Predicate::new_const(
    AOCAT, "has_contributor"
);

/// aocat:has_creator
pub const AOCAT_CREATOR: Predicate = Predicate::new_const(
    AOCAT, "has_creator"
);

/// aocat:has_data_type
pub const AOCAT_DATA_TYPE: Predicate = Predicate::new_const(
    AOCAT, "has_data_type"
);

/// aocat:has_derived_subject
pub const AOCAT_DERIVED_SUBJECT: Predicate = Predicate::new_const(
    AOCAT, "has_derived_subject"
);

/// aocat:has_description
pub const AOCAT_DESCRIPTION: Predicate = Predicate::new_const(
    AOCAT, "has_description"
);

/// aocat:has_landing_page
pub const AOCAT_LANDING_PAGE: Predicate = Predicate::new_const(
    AOCAT, "has_landing_page"
);

/// aocat:has_language
pub const AOCAT_LANGUAGE: Predicate = Predicate::new_const(
    AOCAT, "has_language"
);

/// aocat:has_native_subject
pub const AOCAT_NATIVE_SUBJECT: Predicate = Predicate::new_const(
    AOCAT, "has_native_subject"
);

/// aocat:has_original_id
pub const AOCAT_ORIGINAL_ID: Predicate = Predicate::new_const(
    AOCAT, "has_original_id"
);

/// aocat:has_owner
pub const AOCAT_OWNER: Predicate = Predicate::new_const(
    AOCAT, "has_owner"
);

/// aocat:has_publisher
pub const AOCAT_PUBLISHER: Predicate = Predicate::new_const(
    AOCAT, "has_publisher"
);

/// aocat:has_responsible
pub const AOCAT_RESPONSIBLE: Predicate = Predicate::new_const(
    AOCAT, "has_responsible"
);

/// rdf:type
pub const RDF_TYPE: Predicate = Predicate::new_const(
    RDF, "type"
);

/// rdfs:label
pub const RDFS_LABEL: Predicate = Predicate::new_const(
    RDFS, "label"
);

/// skos:prefLabel
pub const SKOS_PREFLABEL: Predicate = Predicate::new_const(
    SKOS, "prefLabel"
);