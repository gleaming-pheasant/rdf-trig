//! This module contains a collection of statically-declared, commonly used 
//! [`Namespace`]s.
//! 
//! Use the `const` function [`Namespace::new_const`] to create `const` 
//! `Namespace`s for use in your own applications.

/// A macro for declaring modules to contain static - generally widely used -
/// ontologies. This crate creates ontologies for RDF, RDFS, OWL, and AOCAT with 
/// a view to implementing CIDOC_CRM where time permits.
macro_rules! define_ontology {
    ($ontology:ident, $base:literal,
        Properties { $($p_variant:ident => $p_local_name:literal),* $(,)? },
        Classes { $($c_variant:ident => $c_local_name:literal),* $(,)? }
    ) => {
        pub mod $ontology {
            use crate::nodes::{NamedNode, Object, Predicate, Subject};

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub enum Property { $($p_variant),* }

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub enum Class { $($c_variant),* }

            impl Into<Subject<'static>> for Property {
                #[inline]
                fn into(self) -> Subject<'static> {
                    Subject::new_const_named(
                        match self {
                            $(Property::$p_variant => concat!($base, $p_local_name),)*
                        }
                    )
                }
            }

            impl Into<Subject<'static>> for Class {
                #[inline]
                fn into(self) -> Subject<'static> {
                    Subject::new_const_named(
                        match self {
                            $(Class::$c_variant => concat!($base, $c_local_name),)*
                        }
                    )
                }
            }

            impl Into<Predicate<'static>> for Property {
                #[inline]
                fn into(self) -> Predicate<'static> {
                    Predicate::new_const(
                        match self {
                            $(Property::$p_variant => concat!($base, $p_local_name),)*
                        }
                    )
                }
            }

            impl Into<Predicate<'static>> for Class {
                #[inline]
                fn into(self) -> Predicate<'static> {
                    Predicate::new_const(
                        match self {
                            $(Class::$c_variant => concat!($base, $c_local_name),)*
                        }
                    )
                }
            }

            impl Into<Object<'static>> for Property {
                #[inline]
                fn into(self) -> Object<'static> {
                    Object::new_const_named(
                        match self {
                            $(Property::$p_variant => concat!($base, $p_local_name),)*
                        }
                    )
                }
            }

            impl Into<Object<'static>> for Class {
                #[inline]
                fn into(self) -> Object<'static> {
                    Object::new_const_named(
                        match self {
                            $(Class::$c_variant => concat!($base, $c_local_name),)*
                        }
                    )
                }
            }

            impl Into<NamedNode<'static>> for Property {
                #[inline]
                fn into(self) -> NamedNode<'static> {
                    NamedNode::new_const(
                        match self {
                            $(Property::$p_variant => concat!($base, $p_local_name),)*
                        }
                    )
                }
            }

            impl Into<NamedNode<'static>> for Class {
                #[inline]
                fn into(self) -> NamedNode<'static> {
                    NamedNode::new_const(
                        match self {
                            $(Class::$c_variant => concat!($base, $c_local_name),)*
                        }
                    )
                }
            }
        }
    };
}

define_ontology!(aocat, "https://www.ariadne-infrastructure.eu/resource/ao/cat/1.1/",
    Properties {
        HasName => "has_name",
        HasPlaceName => "has_place_name",
        HasDataType => "has_data_type",
        HasCountry => "has_country",
        HasSpatialCoverage => "has_spatial_coverage",
        HasIdentifier => "has_identifier",
        HasType => "has_type",
        IsTypeOf => "is_type_of",
        HasTitle => "has_title",
        HasDescription => "has_description",
        WasIssued => "was_issued",
        WasModified => "was_modified",
        HasPart => "has_part",
        IsPartOf => "is_part_of",
        HasPublisher => "has_publisher",
        IsPublisherOf => "is_publisher_of",
        HasContributor => "has_contributor",
        IsContributorOf => "is_contributor_of",
        HasCreator => "has_creator",
        IsCreatorOf => "is_creator_of",
        HasOwner => "has_owner",
        IsOwnerOf => "is_owner_of",
        HasResponsible => "has_responsible",
        IsResponsibleOf => "is_responsible_of",
        HasVisualComponent => "has_visual_component",
        IsVisualComponentOf => "is_visual_component_of",
        HasPrimaryVisualComponent => "has_primary_visual_component",
        IsPrimaryVisualComponentOf => "is_primary_visual_component_of",
        IsRenderedBy => "is_rendered_by",
        Renders => "renders",
        HasOriginalId => "has_original_id",
        RefersTo => "refers_to",
        IsReferencedBy => "is_referenced_by",
        IsAbout => "is_about",
        IsSubjectOf => "is_subject_of",
        HasAriadneSubject => "has_ARIADNE_subject",
        IsAriadneSubjectOf => "is_ARIADNE_subject_of",
        HasNativeSubject => "has_native_subject",
        IsNativeSubjectOf => "is_native_subject_of",
        HasDerivedSubject => "has_derived_subject",
        IsDerivedSubjectOf => "is_derived_subject_of",
        HasLanguage => "has_language",
        IsLanguageOf => "is_language_of",
        WasCreatedOn => "was_created_on",
        HasLandingPage => "has_landing_page", 
        IsLandingPageOf => "is_landing_page_of",
        HasAccessPolicy => "has_access_policy",
        HasAccessRights => "has_access_rights",
        IsAccessPolicyOf => "is_access_policy_of"
    },
    Classes {
        AoActivity => "AO_Activity",
        AoAgent => "AO_Agent",
        AoCollection => "AO_Collection",
        AoDigitalImage => "AO_Digital_Image",
        AoDigitalMedia => "AO_Digital_Media",
        AoConcept => "AO_Dimension",
        AoDataResource => "AO_Data_Resource",
        AoDocument => "AO_Document",
        AoEntity => "AO_Entity",
        AoEvent => "AO_Event",
        AoGroup => "AO_Group",
        AoIndividualDataResource => "AO_Individual_Data_Resource",
        AoObject => "AO_Object",
        AoPerson => "AO_Person",
        AoResource => "AO_Resource",
        AoService => "AO_Service",
        AoSpatialRegion => "AO_Spatial_Region",
        AoSpatialRegionBBox => "AO_Spatial_Region_BBox",
        AoSpatialRegionPoint => "AO_Spatial_Region_Point",
        AoSpatialRegionPolygon => "AO_Spatial_Region_Polygon",
        AoSpatialRegionStdName => "AO_Spatial_Region_StdName",
        AoTemporalRegion => "AO_Temporal_Region"
    }
);

define_ontology!(owl, "http://www.w3.org/2002/07/owl#",
    Properties {
        AllValuesFrom => "allValuesFrom",
        BackwardCompatibleWith => "backwardCompatibleWith",
        Cardinality => "cardinality",
        ComplementOf => "complementOf",
        DifferentFrom => "differentFrom",
        DisjointWith => "disjointWith",
        DistinctMembers => "distinctMembers",
        EquivalentClass => "equivalentClass",
        EquivalentProperty => "equivalentProperty",
        HasValue => "hasValue",
        Imports => "imports",
        IncompatibleWith => "incompatibleWith",
        IntersectionOf => "intersectionOf",
        InverseOf => "inverseOf",
        MaxCardinality => "maxCardinality",
        MinCardinality => "minCardinality",
        OneOf => "oneOf",
        OnProperty => "onProperty",
        PriorVersion => "priorVersion",
        SameAs => "sameAs",
        SomeValuesFrom => "someValuesFrom",
        UnionOf => "unionOf",
        VersionInfo => "versionInfo"
    },
    Classes {
        AllDifferent => "AllDifferent",
        AnnotationProperty => "AnnotationProperty",
        Class => "Class",
        DatatypeProperty => "DatatypeProperty",
        DeprecatedClass => "DeprecatedClass",
        DeprecatedProperty => "DeprecatedProperty",
        DataRange => "DataRange",
        FunctionalProperty => "FunctionalProperty",
        InverseFunctionalProperty => "InverseFunctionalProperty",
        Nothing => "Nothing",
        ObjectProperty => "ObjectProperty",
        Ontology => "Ontology",
        OntologyProperty => "OntologyProperty",
        Restriction => "Restriction",
        SymmetricProperty => "SymmetricProperty",
        Thing => "Thing",
        TransitiveProperty => "TransitiveProperty"
    }
);

define_ontology!(rdf, "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    Properties {
        First => "first",
        Object => "object",
        Predicate => "predicate",
        Rest => "rest",
        Subject => "subject",
        Type => "type",
        Value => "value"
    },
    Classes {
        Alt => "Alt",
        Bag => "Bag",
        Html => "HTML",
        LangString => "langString", // Yes, non-standard camel case
        List => "List",
        Property => "Property",
        Seq => "Seq",
        Statement => "Statement",
        XmlLiteral => "XMLLiteral"
    }
);

define_ontology!(rdfs, "http://www.w3.org/2000/01/rdf-schema#",
    Properties {
        Comment => "comment",
        Domain => "domain",
        IsDefinedBy => "isDefinedBy",
        Label => "label",
        Member => "member",
        Range => "range",
        SeeAlso => "seeAlso",
        SubClassOf => "subClassOf",
        SubPropertyOf => "subPropertyOf"
    },
    Classes {
        Class => "Class",
        Container => "Container",
        ContainerMembershipProperty => "ContainerMembershipProperty",
        Datatype => "Datatype",
        Literal => "Literal",
        Resource => "Resource"
    }
);

define_ontology!(skos, "http://www.w3.org/2004/02/skos/core#",
    Properties {
        AltLabel => "altLabel",
        PrefLabel => "prefLabel"
    },
    Classes {
        Collection => "Collection",
        Concept => "Concept"
    }
);

#[cfg(test)]
mod tests {
    use crate::nodes::Predicate;

    use super::*;

    #[test]
    fn test_valid_iri() {
        let spl: Predicate<'static> = skos::Property::PrefLabel.into();
        assert_eq!(
            spl,
            Predicate::new_const("http://www.w3.org/2004/02/skos/core#prefLabel")
        )
    }
}