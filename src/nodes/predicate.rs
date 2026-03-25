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

/// aocat:from
pub const AOCAT_FROM: Predicate = Predicate::new_const(NOPE MUST BE IRI);

/// aocat:has_ARIADNE_subject
pub const AOCAT_ARIADNE_SUBJECT: Predicate = Predicate::new_const("has_ARIADNE_subject");

/// aocat:has_access_policy
pub const AOCAT_ACCESS_POLICY: Predicate = Predicate::new_const("has_access_policy");

/// aocat:has_access_rights
pub const AOCAT_ACCESS_RIGHTS: Predicate = Predicate::new_const("has_access_rights");

/// aocat:has_bounding_box_max_lon
pub const AOCAT_BB_MAX_X: Predicate = Predicate::new_const("has_bounding_box_max_lon");

/// aocat:has_bounding_box_max_lat
pub const AOCAT_BB_MAX_Y: Predicate = Predicate::new_const("has_bounding_box_max_lat");

/// aocat:has_bounding_box_min_lon
pub const AOCAT_BB_MIN_X: Predicate = Predicate::new_const("has_bounding_box_min_lon");

/// aocat:has_bounding_box_min_lat
pub const AOCAT_BB_MIN_Y: Predicate = Predicate::new_const("has_bounding_box_min_lat");

/// aocat:has_contributor
pub const AOCAT_CONTRIBUTOR: Predicate = Predicate::new_const("has_contributor");

/// aocat:has_country
pub const AOCAT_COUNTRY: Predicate = Predicate::new_const("has_country");

/// aocat:has_creator
pub const AOCAT_CREATOR: Predicate = Predicate::new_const("has_creator");

/// aocat:has_data_type
pub const AOCAT_DATA_TYPE: Predicate = Predicate::new_const("has_data_type");

/// aocat:has_derived_subject
pub const AOCAT_DERIVED_SUBJECT: Predicate = Predicate::new_const("has_derived_subject");

/// aocat:has_description
pub const AOCAT_DESCRIPTION: Predicate = Predicate::new_const("has_description");

/// aocat:has_identifier
pub const AOCAT_IDENTIFIER: Predicate = Predicate::new_const("has_identifier");

/// aocat:has_landing_page
pub const AOCAT_LANDING_PAGE: Predicate = Predicate::new_const("has_landing_page");

/// aocat:has_language
pub const AOCAT_LANGUAGE: Predicate = Predicate::new_const("has_language");

/// aocat:has_latitude
pub const AOCAT_LATITUDE: Predicate = Predicate::new_const("has_latitude");

/// aocat:has_longitude
pub const AOCAT_LONGITUDE: Predicate = Predicate::new_const("has_longitude");

/// aocat:has_native_subject
pub const AOCAT_NATIVE_SUBJECT: Predicate = Predicate::new_const("has_native_subject");

/// aocat:has_original_id
pub const AOCAT_ORIGINAL_ID: Predicate = Predicate::new_const("has_original_id");

/// aocat:has_owner
pub const AOCAT_OWNER: Predicate = Predicate::new_const("has_owner");

/// aocat:has_period
pub const AOCAT_PERIOD: Predicate = Predicate::new_const("has_period");

/// aocat:has_place_name
pub const AOCAT_PLACE_NAME: Predicate = Predicate::new_const("has_place_name");

/// aocat:has_polygonal_representation
pub const AOCAT_POLYGONAL: Predicate = Predicate::new_const("has_polygonal_representation");

/// aocat:has_native_period
pub const AOCAT_NATIVE_PERIOD: Predicate = Predicate::new_const("has_native_period");

/// aocat:has_publisher
pub const AOCAT_PUBLISHER: Predicate = Predicate::new_const("has_publisher");

/// aocat:has_responsible
pub const AOCAT_RESPONSIBLE: Predicate = Predicate::new_const("has_responsible");

/// aocat:has_spatial_coverage
pub const AOCAT_SPATIAL_COVERAGE: Predicate = Predicate::new_const("has_spatial_coverage");

/// aocat:has_temporal_coverage
pub const AOCAT_TEMPORAL_COVERAGE: Predicate = Predicate::new_const("has_temporal_coverage");

/// aocat:has_title
pub const AOCAT_TITLE: Predicate = Predicate::new_const("has_title");

/// aocat:has_type
pub const AOCAT_TYPE: Predicate = Predicate::new_const("has_type");

/// aocat:has_visual_component
pub const AOCAT_VISUAL_COMPONENT: Predicate = Predicate::new_const("has_visual_component");

/// aocat:is_rendered_by
pub const AOCAT_RENDERED_BY: Predicate = Predicate::new_const("is_rendered_by");

/// aocat:until
pub const AOCAT_UNTIL: Predicate = Predicate::new_const("until");

/// aocat:was_issued
pub const AOCAT_ISSUED: Predicate = Predicate::new_const("was_issued");

/// aocat:was_modified
pub const AOCAT_MODIFIED: Predicate = Predicate::new_const("was_modified");

/// aocat:was_created_on
pub const AOCAT_CREATED_ON: Predicate = Predicate::new_const("was_created_on");

/// aocat:has_primary_visual_component
pub const AOCAT_PRIMARY_VISUAL_COMPONENT: Predicate = Predicate::new_const("has_primary_visual_component");

/// rdf:type
pub const RDF_TYPE: Predicate = Predicate::new_const("type");

/// rdfs:label
pub const RDFS_LABEL: Predicate = Predicate::new_const("label");

/// skos:prefLabel
pub const SKOS_PREFLABEL: Predicate = Predicate::new_const("prefLabel");