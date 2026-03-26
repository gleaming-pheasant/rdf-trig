//! `nodes` is composed of the user-facing nodes [`Subject`], [`Object`] and 
//! [`Predicate`], which are required for constructing triples, but must be 
//! created from the individual types of node; [`BlankNode`], [`IriNode`] and 
//! [`LiteralNode`] (including all of its component options).
//! 
//! Each node type contains either native types of known size or `Cow<'a, str>` 
//! which allow for references to be used to prevent allocation if a node has 
//! already been interned. A 'static or Owned lifetime is coerced on interning 
//! of the node (storing it in a `TripleStore`).
mod blank;
mod graph;
mod literals;
mod named;
mod object;
pub mod predicate; // Public to allow access to const `Predicate`s.
mod subject;
mod store;

pub use blank::BlankNode;
pub use graph::Graph;
pub use named::NamedNode;
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


// Must be an enum not a trait, in order to implement Hash via macro.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Node<'a> {
    Blank(BlankNode<'a>),
    Literal(LiteralNode<'a>),
    Named(NamedNode<'a>)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::WriteNQuads;

    #[test]
    fn test_bool_true_str() {
        let bool_literal = BooleanLiteral::try_from_str("true");
        assert!(bool_literal.is_ok());
        assert!(bool_literal.unwrap().0);
    }

    #[test]
    fn test_bool_false_str_int() {
        let bool_literal = BooleanLiteral::try_from_str("0");
        assert!(bool_literal.is_ok());
        assert!(!bool_literal.unwrap().0);
    }

    #[test]
    fn test_bool_invalid_str() {
        let bool_literal = BooleanLiteral::try_from_str("False");
        assert!(bool_literal.is_err());
    }

    #[test]
    fn test_bool_false_u8() {
        let bool_literal = BooleanLiteral::try_from(0u8);
        assert!(bool_literal.is_ok());
        assert!(!bool_literal.unwrap().0);
    }

    #[test]
    fn test_bool_invalid_u8() {
        let bool_literal = BooleanLiteral::try_from(69u8);
        assert!(bool_literal.is_err());
    }

    #[test]
    fn test_bool_write_trig() {
        let bool_literal = BooleanLiteral::from(true);
        
        let mut buf = vec![];
        bool_literal.write_trig(&mut buf).unwrap();

        assert_eq!(String::from_utf8(buf).unwrap(), String::from("true"));
    }

    #[test]
    fn test_datetime_str_utc_valid() {
        assert!(
            DateTimeLiteral::try_from_str("1969-01-01T12:12:12Z")
            .is_ok()
        );
    }

    #[test]
    fn test_datetime_str_with_tz_valid() {
        assert!(
            DateTimeLiteral::try_from_str("2026-03-02T09:00:00.000+01:00")
            .is_ok()
        );
    }

    #[test]
    fn test_datetime_str_invalid() {
        assert!(
            DateTimeLiteral::try_from_str("Not a datetime")
            .is_err()
        );
    }

    #[test]
    fn test_datetime_str_write_trig() {
        let dt_str = "2020-01-01T12:00:00.000-11:00";
        let dt_literal = DateTimeLiteral::try_from_str(dt_str).unwrap();

        let mut buf = vec![];
        dt_literal.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            format!("\"{}\"^^xsd:dateTime", dt_str)
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_datetime_chrono_naive() {
        let naive = chrono::NaiveDateTime::parse_from_str(
            "2026-03-02T09:00:00.00000", "%Y-%m-%dT%H:%M:%S%.f"
        ).unwrap();

        assert!(DateTimeLiteral::try_from(naive).is_ok());
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_datetime_chrono_offset_write_trig() {
        use chrono::TimeZone;

        // 5 Hours
        let offset = chrono::FixedOffset::east_opt(5 * 3600).unwrap()
            .with_ymd_and_hms(2026, 03, 02, 09, 0, 0).unwrap();
        let offset_node = DateTimeLiteral::try_from(offset).unwrap();

        let mut offset_buf = vec![];
        offset_node.write_trig(&mut offset_buf).unwrap();
        let offset_string = String::from_utf8(offset_buf).unwrap();
        assert_eq!(
            offset_string,
            String::from("\"2026-03-02T09:00:00+05:00\"^^xsd:dateTime")
        );
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_datetime_time_valid_offset() {
        use time::macros::datetime;

        let offset = datetime!(2026-03-02 09:00:00.000 +1);
        assert!(DateTimeLiteral::try_from(offset).is_ok());
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_datetime_time_offset_write_trig() {
        use time::macros::datetime;

        let offset = datetime!(2020-03-01 09:30:25.000 -3);
        let dt_literal = DateTimeLiteral::try_from(offset).unwrap();

        let mut buf = vec![];
        dt_literal.write_trig(&mut buf);

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("\"2020-03-01T09:30:25-03:00\"^^xsd:dateTime")
        )
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_datetime_time_valid_primitive() {
        use time::macros::datetime;

        let primitive = datetime!(2026-01-01 09:00:00.000);
        assert!(DateTimeLiteral::try_from(primitive).is_ok());
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_datetime_time_primitive_write_trig() {
        use time::macros::datetime;

        let primitive = datetime!(2026-01-01 09:59:59.000);
        let dt_literal = DateTimeLiteral::try_from(primitive).unwrap();

        let mut buf = vec![];
        dt_literal.write_trig(&mut buf);

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("\"2026-01-01T09:59:59\"^^xsd:dateTime")
        )
    }

    #[test]
    fn test_decimal_from_str_no_decimal_valid() {
        assert!(DecimalLiteral::try_from_str("69").is_ok());
    }

    #[test]
    fn test_decimal_from_str_valid() {
        assert!(DecimalLiteral::try_from_str("69.420").is_ok());
    }

    #[test]
    fn test_decimal_from_str_write_trig() {
        let decimal_literal = DecimalLiteral::try_from_str("69").unwrap();

        let mut buf = vec![];
        decimal_literal.write_trig(&mut buf).unwrap();
        assert_eq!(
            String::from_utf8(buf).unwrap(), String::from("69.") // Trailing "."
        );
    }

    #[test]
    fn test_decimal_from_native_write_trig() {
        let decimal_literal = DecimalLiteral::from(69.420f32);

        let mut buf = vec![];
        decimal_literal.write_trig(&mut buf).unwrap();

        assert_eq!(String::from_utf8(buf).unwrap(), String::from("69.42"));
    }

    #[test]
    fn test_gyear_from_str_valid() {
        assert!(GYearLiteral::try_from_str("1969").is_ok());
    }

    #[test]
    fn test_gyear_from_str_invalid() {
        assert!(GYearLiteral::try_from_str("Nineteen Sixty Nine").is_err());
    }

    #[test]
    fn test_gyear_from_native_write_trig() {
        let year = -420;
        let gyear_literal = GYearLiteral::from(year);

        let mut buf = vec![];
        gyear_literal.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            format!("\"{}\"^^xsd:gYear", year)
        );
    }

    #[test]
    fn test_lang_string_valid_lang() {
        assert!(LangStringLiteral::new("My String", "fra").is_ok());
    }

    #[test]
    fn test_lang_string_invalid_lang() {
        assert!(LangStringLiteral::new("My String", "francais").is_err());
    }

    #[test]
    fn test_lang_string_valid_lang_write_trig() {
        let value = "My String";
        let langstring_literal = LangStringLiteral::new(value, "gr")
            .unwrap();

        let mut buf = vec![];
        langstring_literal.write_trig(&mut buf).unwrap();
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            format!("\"{}\"@gr", value)
        );
    }

    #[test]
    fn test_lang_string_en_write_trig() {
        let value = "My String";
        let lang_string_literal = LangStringLiteral::new_en(value);

        let mut buf = vec![];
        lang_string_literal.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            format!("\"{}\"@en", value)
        );
    }

    #[test]
    fn test_string_write_trig() {
        let value = "I like turtles";
        let string_literal = LiteralNode::new(&*value);

        let mut buf = vec![];
        string_literal.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            format!("\"{}\"", value)
        );
    }

    #[test]
    fn test_string_write_trig_escaped() {
        let string_literal = LiteralNode::new("I\rlike\tescaped\nturtles");

        let mut buf = vec![];
        string_literal.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            format!("\"{}\"", r"I\rlike\tescaped\nturtles")
        );
    }
}