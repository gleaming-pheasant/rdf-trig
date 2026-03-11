//! `nodes` is composed of the user-facing nodes [`Subject`], [`Object`] and 
//! [`Predicate`], which are required for constructing triples, but must be 
//! created from the individual types of node; [`BlankNode`], [`IriNode`] and 
//! [`LiteralNode`] (including all of its component options).
//! 
//! Each node type contains either native types of known size or `Cow<'a, str>` 
//! which allow for references to be used to prevent allocation if a node has 
//! already been interned. A 'static or Owned lifetime is coerced on interning 
//! of the node (storing it in a `DataStore`).
mod blank;
mod iri;
mod literals;
mod object;
pub mod predicate; // Public to allow access to const `Predicate`s.
mod subject;
mod store;

pub use blank::BlankNode;
pub use iri::IriNode;
pub(crate) use iri::StagingIriNode;
pub use literals::{
    BooleanLiteral,
    DecimalLiteral,
    DateTimeLiteral,
    GYearLiteral,
    LangStringLiteral,
    LiteralNode
};
pub use object::Object;
pub use predicate::Predicate;
pub use subject::Subject;
pub(crate) use store::{NodeId, NodeStore};

use crate::traits::ToInterned;

/// An enumerator over the three node types used in RDF: blank, IRI and literal.
#[derive(Debug)]
pub(crate) enum Node<'a> {
    Blank(BlankNode<'a>),
    Iri(IriNode<'a>),
    Literal(LiteralNode<'a>)
}

/// Serves as a wrapper around the same types as [`Node`], with the exception 
/// that the Iri variant is a [`StagedIriNode`], that is one which has already 
/// retrieved the [`NamespaceId`] for its interned [`Namespace`].
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum StagingNode<'a> {
    Blank(BlankNode<'a>),
    Iri(StagingIriNode<'a>),
    Literal(LiteralNode<'a>)
}

impl<'a> ToInterned for StagingNode<'a> {
    type InternedType = StagingNode<'static>;

    fn to_interned(&self) -> Self::InternedType {
        match self {
            StagingNode::Blank(blank) => {
                StagingNode::Blank(blank.to_interned())
            },
            StagingNode::Iri(iri) => StagingNode::Iri(iri.to_interned()),
            StagingNode::Literal(literal) => {
                StagingNode::Literal(literal.to_interned())
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::traits::WriteTriG;


//     #[test]
//     fn test_true_bool_from_string() {
//         let good_true_one = "true";
//         assert!(LiteralNode::boolean(good_true_one).is_ok());
//     }

//     #[test]
//     fn test_true_bool_from_number() {
//         let good_true_two = String::from("1");
//         assert!(LiteralNode::boolean(good_true_two).is_ok());
//     }

//     #[test]
//     fn test_bad_true_bool() {
//         let bad_true_one = String::from("True");
//         assert!(LiteralNode::boolean(bad_true_one).is_err());
//     }

//     #[test]
//     fn test_invalid_string_bool() {
//         let bad_true_two = "A completely random str.";
//         assert!(LiteralNode::boolean(bad_true_two).is_err());
//     }

//     #[test]
//     fn test_true_bool_from_native() {
//         let raw_true = true;
//         let raw_true_built = LiteralNode::from(raw_true);
//         assert_eq!(raw_true_built, LiteralNode::Boolean(true));
//     }

//     #[test]
//     fn test_true_write_trig() {
//         let raw_true = LiteralNode::Boolean(true);

//         let mut buf: Vec<u8> = vec![];
//         raw_true.write_trig(&mut buf).unwrap();

//         let as_string = String::from_utf8(buf).unwrap();
//         assert_eq!(as_string, String::from("true"));
//     }

//     #[test]
//     fn test_false_bool_from_string() {
//         let good_false_one = "false";
//         assert!(LiteralNode::boolean(good_false_one).is_ok());
//     }

//     #[test]
//     fn test_false_bool_from_number_string() {
//         let good_false_two = String::from("0");
//         assert!(LiteralNode::boolean(good_false_two).is_ok());
//     }

//     #[test]
//     fn test_false_bool_from_native() {
//         let raw_false = false;
//         let raw_false_built = LiteralNode::from(raw_false);
//         assert_eq!(raw_false_built, LiteralNode::Boolean(false));
//     }

//     #[test]
//     fn test_false_write_trig() {
//         let raw_false = LiteralNode::Boolean(false);

//         let mut buf: Vec<u8> = vec![];
//         raw_false.write_trig(&mut buf).unwrap();

//         let as_string = String::from_utf8(buf).unwrap();
//         assert_eq!(as_string, String::from("false"));
//     }

//     #[test]
//     fn test_datetime_from_utc_string() {
//         assert!(LiteralNode::datetime(
//             // String to test Into<Cow<...>> also.
//             String::from("2026-03-02T09:00:00.000Z")
//         ).is_ok());
//     }

//     #[test]
//     fn test_datetime_from_utc_string_no_secs() {
//         assert!(LiteralNode::datetime("2026-03-02T09:00:00Z").is_ok());
//     }

//     #[test]
//     fn test_datetime_with_explicit_tz() {
//         assert!(LiteralNode::datetime("2026-03-02T09:00:00.000+01:00").is_ok());
//     }

//     #[test]
//     fn test_datetime_with_no_tz_or_utc() {
//         assert!(LiteralNode::datetime("2026-03-02T09:00:00").is_ok());
//     }

//     #[test]
//     fn test_invalid_datetime() {
//         assert!(LiteralNode::datetime("Random string").is_err())
//     }
    
//     #[cfg(feature = "time")]
//     #[test]
//     fn test_time_primitive_datetime_try_from() {
//         use time::macros::datetime;

//         let primitive = datetime!(2026-03-02 09:00:00.000);
//         assert!(LiteralNode::try_from(primitive).is_ok());
//     }

//     #[cfg(feature = "time")]
//     #[test]
//     fn test_time_offset_datetime_try_from() {
//         use time::macros::datetime;

//         let offset = datetime!(2026-03-02 09:00:00.000 +1);
//         assert!(LiteralNode::try_from(offset).is_ok());
//     }

//     #[cfg(feature = "time")]
//     #[test]
//     fn test_time_primitive_write_trig() {
//         use time::macros::datetime;

//         let primitive_node = LiteralNode::try_from(
//             datetime!(2026-03-02 09:00:00.000)
//         ).unwrap();

//         let mut primitive_buf = vec![];
//         primitive_node.write_trig(&mut primitive_buf).unwrap();
//         let primitive_string = String::from_utf8(primitive_buf).unwrap();

//         assert_eq!(
//             primitive_string,
//             String::from("\"2026-03-02T09:00:00\"^^xsd:dateTime")
//         );
//     }

//     #[cfg(feature = "time")]
//     #[test]
//     fn test_time_offset_write_trig() {
//         use time::macros::datetime;

//         let offset_node = LiteralNode::try_from(
//             datetime!(2026-03-02 09:00:00.000 +1)
//         ).unwrap();

//         let mut offset_buf = vec![];
//         offset_node.write_trig(&mut offset_buf).unwrap();
//         let offset_string = String::from_utf8(offset_buf).unwrap();
//         assert_eq!(
//             offset_string,
//             String::from("\"2026-03-02T09:00:00+01:00\"^^xsd:dateTime")
//         );
//     }

//     #[cfg(feature = "chrono")]
//     #[test]
//     fn test_chrono_naive_write_trig() {
//         let naive = chrono::NaiveDateTime::parse_from_str(
//             "2026-03-02T09:00:00.00000", "%Y-%m-%dT%H:%M:%S%.f"
//         ).unwrap();
//         let naive_node = LiteralNode::from(naive);

//         let mut naive_buf = vec![];
//         naive_node.write_trig(&mut naive_buf).unwrap();
//         let naive_string = String::from_utf8(naive_buf).unwrap();
//         println!("{naive_string}");
//         assert_eq!(
//             naive_string,
//             String::from("\"2026-03-02T09:00:00\"^^xsd:dateTime")
//         );
//     }

//     #[cfg(feature = "chrono")]
//     #[test]
//     fn test_chrono_datetime_offset_write_trig() {
//         use chrono::TimeZone;

//         let offset = chrono::FixedOffset::east_opt(5 * 3600).unwrap()
//             .with_ymd_and_hms(2026, 03, 02, 09, 0, 0).unwrap();
//         let offset_node = LiteralNode::try_from(offset).unwrap();

//         let mut offset_buf = vec![];
//         offset_node.write_trig(&mut offset_buf).unwrap();
//         let offset_string = String::from_utf8(offset_buf).unwrap();
//         assert_eq!(
//             offset_string,
//             String::from("\"2026-03-02T09:00:00+05:00\"^^xsd:dateTime")
//         );
//     }

//     #[test]
//     fn test_decimal_from_int_string() {
//         assert!(LiteralNode::decimal("69").is_ok());
//     }

//     #[test]
//     fn test_decimal_from_decimal_string() {
//         assert!(LiteralNode::decimal("69.420").is_ok());
//     }

//     #[test]
//     fn test_decimal_from_signed_int_string() {
//         assert!(LiteralNode::decimal("+24").is_ok());
//     }

//     #[test]
//     fn test_decimal_from_signed_decimal_string() {
//         assert!(LiteralNode::decimal("-2.468").is_ok());
//     }

//     #[test]
//     fn test_decimal_from_f32() {
//         assert_eq!(
//             LiteralNode::Decimal(Cow::Owned(String::from("69.42"))),
//             LiteralNode::from(69.420f32)
//         );
//     }
    
//     #[test]
//     fn test_valid_gyear_signed() {
//         assert!(LiteralNode::gyear("-0099").is_ok());
//     }

//     #[test]
//     fn test_valid_gyear_really_old() {
//         assert!(LiteralNode::gyear("-4206969").is_ok());
//     }

//     #[test]
//     fn test_valid_gyear_far_future() {
//         assert!(LiteralNode::gyear("696969").is_ok());
//     }

//     #[test]
//     fn test_valid_gyear_unsigned() {
//         assert!(LiteralNode::gyear("1999").is_ok());
//     }

//     #[test]
//     fn test_valid_gyear_utc() {
//         assert!(LiteralNode::gyear("-0069Z").is_ok());
//     }

//     #[test]
//     fn test_valid_gyear_offset() {
//         assert!(LiteralNode::gyear("-0069+12:00").is_ok());
//     }

//     #[test]
//     fn test_valid_gyear_invalid_offset() {
//         assert!(LiteralNode::gyear("-0069+12:00:59").is_err());
//     }

//     #[test]
//     fn test_invalid_gyear_too_short() {
//         assert!(LiteralNode::gyear("-420").is_err());
//     }

//     #[test]
//     fn test_invalid_gyear_too_short_unsigned() {
//         assert!(LiteralNode::gyear("69").is_err());
//     }

//     #[test]
//     fn test_valid_gyear_from_i32_unsigned_write_trig() {
//         let gyear_node = LiteralNode::gyear_from_i32(69);

//         let mut buf = vec![];
//         gyear_node.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from("\"0069\"^^xsd:gYear")
//         );
//     }

//     #[test]
//     fn test_valid_gyear_from_i32_signed_write_trig() {
//         let gyear_node = LiteralNode::gyear_from_i32(-420);

//         let mut buf = vec![];
//         gyear_node.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from("\"-0420\"^^xsd:gYear")
//         );
//     }

//     #[test]
//     fn test_valid_language_string() {
//         assert!(LiteralNode::string(
//             Some("fr"), String::from("oui")).is_ok()
//         )
//     }

//     #[test]
//     fn test_invalid_language_string() {
//         assert!(LiteralNode::string(
//             Some("french"), String::from("non")).is_ok()
//         )
//     }

//     #[test]
//     fn test_no_language_string() {
//         assert!(LiteralNode::string(
//             None::<String>, String::from("random string")).is_ok()
//         )
//     }

//     #[test]
//     fn test_no_language_string_write_trig() {
//         let node = LiteralNode::string_no_lang("My Literal");

//         let mut buf = vec![];
//         node.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from("\"My Literal\"")
//         );
//     }

//     #[test]
//     fn test_en_language_string_write_trig() {
//         let node = LiteralNode::string_en("My Literal");

//         let mut buf = vec![];
//         node.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from("\"My Literal\"@en")
//         );
//     }

//     #[test]
//     fn test_custom_language_string_write_trig() {
//         let node = LiteralNode::string(
//             Some("eng"), "My Literal"
//         ).unwrap();

//         let mut buf = vec![];
//         node.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from("\"My Literal\"@eng")
//         );
//     }

//     #[test]
//     fn test_custom_language_string_write_trig_with_escape() {
//         let node = LiteralNode::string(
//             Some("uk"), "My\r\nLiteral"
//         ).unwrap();

//         let mut buf = vec![];
//         node.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from("\"My\r\nLiteral\"@uk")
//         );
//     }

//     #[test]
//     fn test_blank_node_write_trig() {
//         let blank = BlankNode::new("myprefix");

//         let mut buf = vec![];
//         blank.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from("_:myprefix")
//         )
//     }

//     #[test]
//     fn test_blank_node_write_trig_w_escape_char() {
//         let blank = BlankNode::new("my_pre~fix\n");

//         let mut buf = vec![];
//         blank.write_trig(&mut buf).unwrap();

//         assert_eq!(
//             String::from_utf8(buf).unwrap(),
//             String::from(r"_:my_pre\~fix")
//         )
//     }

//     #[test]
//     fn test_add_invalid_namespace() {
//         let subject = Subject::iri_with_new_namespace(
//             "badOwl", "can't find owl schema", "Class"
//         );

//         assert!(subject.is_err());
//     }
// }