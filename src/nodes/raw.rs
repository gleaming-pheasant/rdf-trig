use std::borrow::Cow;
use std::io::{Result as IoResult, Write};

use time::{OffsetDateTime, PrimitiveDateTime};
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};

use crate::errors::RdfTrigError;
use crate::namespaces::{Namespace, NamespaceId};
use crate::traits::WriteTriG;
use crate::utils::write_trig_escaped_local_name;

/// These `const`s allow compile-time format descriptions for validating 
/// [`time::PrimitiveDateTime`]s. ISO-3339 formats are tested first, but these 
/// provide a fallback in the event an offset (or "Z") is missing.
/// 
/// `dateTimes` are still valid hese are still valid XML Schema even if they are 
/// missing this offset or UTC identifier.
const FMT_NAIVE_SUBSECOND: &[time::format_description::FormatItem<'_>] = 
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]");
const FMT_NAIVE_ISO: &[time::format_description::FormatItem<'_>] = 
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

/// An `IriNode` is composed of a [`Namespace`] (to allow assigning the iri to a 
/// shared iri using a `prefix`) and an `endpoint`.
/// 
/// These must be instantiated with the [`Subject`], [`Predicate`] or [`Object`] 
/// types directly, to prevent invalid nodes being used in the wrong locations 
/// in a [`Triple`](crate::groups::triples::Triple).
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct IriNode {
    namespace: Namespace,
    endpoint: Cow<'static, str>
}

impl IriNode {
    /// Create a new [`IriNode`].
    pub(crate) fn new<C: Into<Cow<'static, str>>>(
        namespace: Namespace, endpoint: C
    ) -> IriNode {
        IriNode { namespace, endpoint: endpoint.into() }
    }

    pub(crate) fn new_with_new_namespace<P, I, C>(
        prefix: P, iri: I, endpoint: C
    ) -> IriNode
    where
        P: Into<Cow<'static, str>>,
        I: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>
    {
        IriNode {
            namespace: Namespace::new(prefix, iri),
            endpoint: endpoint.into()
        }
    }

    /// Allows you to create a new `IriNode` which is composed of static values 
    /// known as compile time, exported via [`Predicate`](crate::nodes::Predicate).
    pub(crate) const fn new_const(
        namespace: Namespace, endpoint: &'static str
    ) -> IriNode {
        IriNode { namespace, endpoint: Cow::Borrowed(endpoint) }
    }

    /// Consume this `IriNode`, returning a tuple of its `namespace` and 
    /// `endpoint`.
    pub(crate) fn into_parts(self) -> (Namespace, Cow<'static, str>) {
        (self.namespace, self.endpoint)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct InternedIriNode {
    namespace_id: NamespaceId,
    endpoint: Cow<'static, str>
}

impl InternedIriNode {
    /// Create a new [`InternedIriNode`].
    pub(crate) fn new(
        namespace_id: NamespaceId, endpoint: Cow<'static, str>
    ) -> InternedIriNode {
        InternedIriNode { namespace_id, endpoint }
    }

    /// Get the `namespace_id` for this `InternedIriNode`.
    pub(crate) fn namespace_id(&self) -> NamespaceId {
        self.namespace_id
    }

    /// Get a reference to the `endpoint` for this `InternedIriNode`.
    pub(crate) fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

/// A `BlankNode` is a standard RDF blank node. It serves as a a place to store 
/// known facts about a resource within a graph, without knowing the resource's 
/// specific IRI.
/// 
/// `BlankNode` directly implements [`WriteTriG`], prefixing the provided id 
/// with the standard blank node "_:" prefix.
/// 
/// `BlankNode`s cannot be initialised directly, and must be generated as part 
/// of [`Subject`] or [`Object`] constructors.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BlankNode(Cow<'static, str>);

impl BlankNode {
    /// Create a new `BlankNode` with the provided `id`.
    pub(crate) fn new<C: Into<Cow<'static, str>>>(id: C) -> BlankNode {
        BlankNode(id.into())
    }
}

impl WriteTriG for BlankNode {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(b"_:")?;
        write_trig_escaped_local_name(writer, &self.0)?;

        Ok(())
    }
}

/// A `LiteralNode` is an enumerator over xsd literal types, such as "strings" 
/// (with optional language tags), "datetimes" and "gYears".
/// 
/// Because there is nothing to explicitly intern in a `LiteralNode`, this type 
/// directly implements the [`WriteTriG`] trait for TriG formatting.
/// 
/// This enum is __non_exhaustive__, with additional XML Schema types not 
/// currently planned.
#[derive(Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LiteralNode {
    Boolean(bool),
    Datetime(Cow<'static, str>),
    Decimal(Cow<'static, str>),
    GYear(Cow<'static, str>),
    String(StringLiteral)
}

impl LiteralNode {
    /// Declare a `LiteralNode::Boolean` from the provided value.
    /// 
    /// Returns an `RdfTrigError::InvalidBoolean` if the provided value cannot 
    /// be parsed as an XSD boolean ("true", "false", "1", "0").
    /// 
    /// Though the input will eventually be written with [`Write`], this type 
    /// casts the input value to a native [`bool`], for two reasons: the cost of 
    /// conversion to a single byte is acceptable, and the saving of not having 
    /// to write the output with the full `"xsd::boolean"` suffix is considered 
    /// acceptable.
    /// 
    /// For ease, `LiteralNode` also implements [`From<bool>`] for quick 
    /// conversions.
    pub(crate) fn boolean<C: Into<Cow<'static, str>>>(value: C)
    -> Result<LiteralNode, RdfTrigError> {
        let cow_val: Cow<'static, str> = value.into();

        match &*cow_val {
            "true" | "1" => Ok(LiteralNode::Boolean(true)),
            "false" | "0" => Ok(LiteralNode::Boolean(false)),
            _ => Err(RdfTrigError::InvalidBoolean(cow_val))
        }        
    }

    /// Declare a `LiteralNode::Datetime` from the provided value.
    /// 
    /// Returns an `RdfTrigError::InvalidDateTime` if the provided value cannot 
    /// be parsed as an XML Schema `dateTime` ("1900-01-01T00:00:00.000", with 
    /// or without "Z" or a timezone offset).
    /// 
    /// This crate uses `time` to validate XSD `dateTime`s, but implements 
    /// [`From<>`] for [`chrono::DateTime`], [`chrono::NaiveDateTime`], 
    /// [`time::OffsetDateTime`] and [`time::PrimitiveDateTime`] with the 
    /// relevant `chrono` or `time` feature flags enabled.
    pub(crate) fn datetime<C: Into<Cow<'static, str>>>(value: C)
    -> Result<LiteralNode, RdfTrigError> {
        let cow_val: Cow<'static, str> = value.into();

        if OffsetDateTime::parse(&cow_val, &Rfc3339).is_ok() {
            return Ok(LiteralNode::Datetime(cow_val));
        }

        if PrimitiveDateTime::parse(
            &cow_val, &FMT_NAIVE_SUBSECOND
        ).is_ok() || PrimitiveDateTime::parse(
            &cow_val, &FMT_NAIVE_ISO
        ).is_ok() {
            Ok(LiteralNode::Datetime(cow_val))
        } else {
            Err(RdfTrigError::InvalidDateTime(cow_val))
        }
    }

    /// Declare a `LiteralNode::Decimal` type from the provided value.
    /// 
    /// Returns an `RdfTrigError::InvalidDecimal` if the provided value cannot 
    /// be parsed as an `f32`.
    /// 
    /// For ease, `LiteralNode` also implements [`From<f32>`] for quick 
    /// conversions.
    pub(crate) fn decimal<C: Into<Cow<'static, str>>>(value: C)
    -> Result<LiteralNode, RdfTrigError> {
        // Deliberately does not drop the `str` in place of the f32 at any 
        // point, as the crate would only have to return it to that format for 
        // io::Write.
        let cow_val: Cow<'static, str> = value.into();

        match cow_val.parse::<f32>() {
            Ok(_) => Ok(LiteralNode::Decimal(cow_val)),
            Err(_) => Err(RdfTrigError::InvalidDecimal(cow_val))
        }
    }

    /// Declare a `LiteralNode::GYear` type from the provided value.
    /// 
    /// Returns an `RdfTrigError::InvalidGYear` if the provided value cannot be 
    /// parsed as an XSD gYear (CE/BCE year, with or without a timezone offset).
    pub(crate) fn gyear<C: Into<Cow<'static, str>>>(value: C)
    -> Result<LiteralNode, RdfTrigError> {
        let cow_val: Cow<'static, str> = value.into();
        let bytes = cow_val.as_bytes();
        let len = bytes.len(); // Saved as used repeatedly.

        if len < 4 { return Err(RdfTrigError::InvalidGYear(cow_val)) }

        let mut cursor = 0;

        if bytes[cursor] == b'-' {
            cursor += 1;
        }

        let year_start = cursor; // 0 if no sign, 1 if signed.
        
        // Breaks if encounters a character that isn't a digit or reaches end.
        while cursor < len && bytes[cursor].is_ascii_digit() {
            cursor += 1;
        }

        if len - year_start < 4 {
            // Still not long enough, so invalid.
            return Err(RdfTrigError::InvalidGYear(cow_val));
        }

        if cursor < len {
            let remaining = &bytes[cursor..];
            match remaining {
                // One character, "Z" means UTC.
                [b'Z'] => cursor += 1,
                // 6 characters in a valid format (eg. "+01:00").
                [sign @ (b'+' | b'-'), h1, h2, b':', m1, m2] 
                    if h1.is_ascii_digit() && h2.is_ascii_digit() 
                    && m1.is_ascii_digit() && m2.is_ascii_digit() => {
                    cursor += 6;
                }
                _ => return Err(RdfTrigError::InvalidGYear(cow_val)),
            }
        }

        // Check entire string has been parsed.
        if cursor == len {
            Ok(LiteralNode::GYear(cow_val))
        } else {
            // Too long/too many characters
            Err(RdfTrigError::InvalidDateTime(cow_val))
        }
    }

    /// Create a new `LiteralNode::GYear` with the provided [`i32`].
    /// 
    /// This function pads the year by padding the `i32` with preceding zeroes.
    /// 
    /// Unfortunately, while it would be quicker to store the value as an `i32`, 
    /// and write the padding on [`Write`], the requirement to accept `str` 
    /// types to allow gYears to have timezones means that the formatting must 
    /// happen here to match the same type.
    pub(crate) fn gyear_from_i32(value: i32) -> LiteralNode {
        let formatted_gyear = if value < 0 {
            format!("-{:0>4}", value.unsigned_abs())
        } else {
            format!("{:0>4}", value)
        };

        LiteralNode::GYear(Cow::Owned(formatted_gyear))
    }

    /// Create a new `LiteralNode::String` with the provided `language` and 
    /// string `value`.
    /// 
    /// Returns an `RdfTrigError::InvalidLanguage` if the provided language is 
    /// not a valid ISO-639 language code.
    pub(crate) fn string<L, V>(
        language: Option<L>, value: V
    ) -> Result<LiteralNode, RdfTrigError>
    where
        L: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>
    {
        let lang_cow_opt = language.map(Into::into);

        if let Some(ref lang_cow) = lang_cow_opt {
            let bytes = lang_cow.as_bytes();
            if !(bytes.len() == 2 || bytes.len() == 3) 
            && !bytes.iter().all(u8::is_ascii_lowercase) {
                // Unwrap is safe as is within if let Some(var) clause.
                return Err(RdfTrigError::InvalidLanguage(lang_cow_opt.unwrap()))
            }
        }

        Ok(LiteralNode::String(StringLiteral {
            language: lang_cow_opt, value: value.into()
        }))
    }

    /// Create a new `LiteralNode::String` with the `language` code already set 
    /// to "en" for English.
    pub(crate) fn string_en<V: Into<Cow<'static, str>>>(
        value: V
    ) -> LiteralNode {
        LiteralNode::String(StringLiteral::new_en(value))
    }

    /// Create a new `LiteralNode::String` with the `language` code set to 
    /// `None`.
    pub(crate) fn string_no_lang<V: Into<Cow<'static, str>>>(
        value: V
    ) -> LiteralNode {
        LiteralNode::String(StringLiteral::new_no_lang(value))
    }
}

impl From<bool> for LiteralNode {
    fn from(value: bool) -> Self {
        LiteralNode::Boolean(value)
    }
}

impl From<f32> for LiteralNode {
    fn from(value: f32) -> Self {
        LiteralNode::Decimal(Cow::Owned(value.to_string()))
    }
}


#[cfg(feature = "chrono")]
impl From<NaiveDateTime> for LiteralNode {
    fn from(value: NaiveDateTime) -> LiteralNode {
        LiteralNode::Datetime(
            Cow::Owned(value.format("%Y-%m-%dT%H:%M:%S").to_string())
        )
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<FixedOffset>> for LiteralNode {
    fn from(value: DateTime<FixedOffset>) -> Self {
        LiteralNode::Datetime(
            Cow::Owned(value.format("%+").to_string())
        )
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<Local>> for LiteralNode {
    fn from(value: DateTime<Local>) -> Self {
        LiteralNode::Datetime(
            Cow::Owned(value.format("%+").to_string())
        )
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<Utc>> for LiteralNode {
    fn from(value: DateTime<Utc>) -> Self {
        LiteralNode::Datetime(
            Cow::Owned(value.format("%+").to_string())
        )
    }
}

#[cfg(feature = "time")]
impl TryFrom<PrimitiveDateTime> for LiteralNode {
    type Error = RdfTrigError;

    fn try_from(value: PrimitiveDateTime) -> Result<Self, Self::Error> {
        let fmt = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

        match value.format(&fmt) {
            Ok(dt_str) => Ok(LiteralNode::Datetime(Cow::Owned(dt_str))),
            Err(_) => {
                Err(RdfTrigError::InvalidDateTime(
                        "Invalid PrimitiveDateTime".into()
                ))
            }
        }
    }
}

#[cfg(feature = "time")]
impl TryFrom<OffsetDateTime> for LiteralNode {
    type Error = RdfTrigError;

    fn try_from(value: OffsetDateTime) -> Result<Self, Self::Error> {
        match value.format(&Rfc3339) {
            Ok(dt_str) => Ok(LiteralNode::Datetime(Cow::Owned(dt_str))),
            Err(_) => {
                Err(RdfTrigError::InvalidDateTime(
                        "Invalid OffsetDateTime".into()
                ))
            }
        }
    }
}

impl WriteTriG for LiteralNode {
    fn write_trig<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        match self {
            LiteralNode::Boolean(b) => {
                writer.write_all(b.to_string().as_bytes())?;
            },
            LiteralNode::Datetime(dt) => {
                writer.write_all(b"\"")?;
                writer.write_all(dt.as_bytes())?;
                writer.write_all(b"\"^^xsd:dateTime")?;
            },
            LiteralNode::Decimal(dec) => {
                writer.write_all(dec.to_string().as_bytes())?;
            }
            LiteralNode::GYear(gy) => {
                writer.write_all(b"\"")?;
                writer.write_all(gy.as_bytes())?;
                writer.write_all(b"\"^^xsd:gYear")?;
            },
            LiteralNode::String(st) => {
                match st.language() {
                    Some(lang) => {
                        write!(
                            writer, "\"{}\"@{}",
                            st.value(), lang
                        )?;},
                    None => {
                        writer.write_all(b"\"")?;
                        writer.write_all(st.value().as_bytes())?;
                        writer.write_all(b"\"")?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StringLiteral {
    language: Option<Cow<'static, str>>,
    value: Cow<'static, str>
}

impl StringLiteral {
    /// Create a new `StringLiteral` from a `language` tag and `value`.
    pub(crate) fn new(
        language: Option<Cow<'static, str>>, value: Cow<'static, str>
    ) -> StringLiteral {
        StringLiteral {
            language: language,
            value: value.into()
        }
    }

    /// Create a new `StringLiteral` with the `language` set to Some("en").
    pub(crate) fn new_en<V: Into<Cow<'static, str>>>(value: V) -> StringLiteral {
        StringLiteral { language: Some("en".into()), value: value.into() }
    }

    /// Create a new `StringLiteral` with the `language` set to `None`.
    pub(crate) fn new_no_lang<V: Into<Cow<'static, str>>>(
        value: V
    ) -> StringLiteral {
        StringLiteral { language: None, value: value.into() }
    }

    /// Return a reference to this `StringLiteral`'s `language`.
    pub(crate) fn language(&self) -> &Option<Cow<'static, str>> {
        &self.language
    }

    /// Return a reference to this `StringLiteral`'s `value`.
    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum InternedNode {
    Blank(BlankNode),
    Iri(InternedIriNode),
    Literal(LiteralNode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::WriteTriG;


    #[test]
    fn test_true_bool_from_string() {
        let good_true_one = "true";
        assert!(LiteralNode::boolean(good_true_one).is_ok());
    }

    #[test]
    fn test_true_bool_from_number() {
        let good_true_two = String::from("1");
        assert!(LiteralNode::boolean(good_true_two).is_ok());
    }

    #[test]
    fn test_bad_true_bool() {
        let bad_true_one = String::from("True");
        assert!(LiteralNode::boolean(bad_true_one).is_err());
    }

    #[test]
    fn test_invalid_string_bool() {
        let bad_true_two = "A completely random str.";
        assert!(LiteralNode::boolean(bad_true_two).is_err());
    }

    #[test]
    fn test_true_bool_from_native() {
        let raw_true = true;
        let raw_true_built = LiteralNode::from(raw_true);
        assert_eq!(raw_true_built, LiteralNode::Boolean(true));
    }

    #[test]
    fn test_true_write_trig() {
        let raw_true = LiteralNode::Boolean(true);

        let mut buf: Vec<u8> = vec![];
        raw_true.write_trig(&mut buf).unwrap();

        let as_string = String::from_utf8(buf).unwrap();
        assert_eq!(as_string, String::from("true"));
    }

    #[test]
    fn test_false_bool_from_string() {
        let good_false_one = "false";
        assert!(LiteralNode::boolean(good_false_one).is_ok());
    }

    #[test]
    fn test_false_bool_from_number_string() {
        let good_false_two = String::from("0");
        assert!(LiteralNode::boolean(good_false_two).is_ok());
    }

    #[test]
    fn test_false_bool_from_native() {
        let raw_false = false;
        let raw_false_built = LiteralNode::from(raw_false);
        assert_eq!(raw_false_built, LiteralNode::Boolean(false));
    }

    #[test]
    fn test_false_write_trig() {
        let raw_false = LiteralNode::Boolean(false);

        let mut buf: Vec<u8> = vec![];
        raw_false.write_trig(&mut buf).unwrap();

        let as_string = String::from_utf8(buf).unwrap();
        assert_eq!(as_string, String::from("false"));
    }

    #[test]
    fn test_datetime_from_utc_string() {
        assert!(LiteralNode::datetime(
            // String to test Into<Cow<...>> also.
            String::from("2026-03-02T09:00:00.000Z")
        ).is_ok());
    }

    #[test]
    fn test_datetime_from_utc_string_no_secs() {
        assert!(LiteralNode::datetime("2026-03-02T09:00:00Z").is_ok());
    }

    #[test]
    fn test_datetime_with_explicit_tz() {
        assert!(LiteralNode::datetime("2026-03-02T09:00:00.000+01:00").is_ok());
    }

    #[test]
    fn test_datetime_with_no_tz_or_utc() {
        assert!(LiteralNode::datetime("2026-03-02T09:00:00").is_ok());
    }

    #[test]
    fn test_invalid_datetime() {
        assert!(LiteralNode::datetime("Random string").is_err())
    }
    
    #[cfg(feature = "time")]
    #[test]
    fn test_time_primitive_datetime_try_from() {
        use time::macros::datetime;

        let primitive = datetime!(2026-03-02 09:00:00.000);
        assert!(LiteralNode::try_from(primitive).is_ok());
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_time_offset_datetime_try_from() {
        use time::macros::datetime;

        let offset = datetime!(2026-03-02 09:00:00.000 +1);
        assert!(LiteralNode::try_from(offset).is_ok());
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_time_primitive_write_trig() {
        use time::macros::datetime;

        let primitive_node = LiteralNode::try_from(
            datetime!(2026-03-02 09:00:00.000)
        ).unwrap();

        let mut primitive_buf = vec![];
        primitive_node.write_trig(&mut primitive_buf).unwrap();
        let primitive_string = String::from_utf8(primitive_buf).unwrap();

        assert_eq!(
            primitive_string,
            String::from("\"2026-03-02T09:00:00\"^^xsd:dateTime")
        );
    }

    #[cfg(feature = "time")]
    #[test]
    fn test_time_offset_write_trig() {
        use time::macros::datetime;

        let offset_node = LiteralNode::try_from(
            datetime!(2026-03-02 09:00:00.000 +1)
        ).unwrap();

        let mut offset_buf = vec![];
        offset_node.write_trig(&mut offset_buf).unwrap();
        let offset_string = String::from_utf8(offset_buf).unwrap();
        assert_eq!(
            offset_string,
            String::from("\"2026-03-02T09:00:00+01:00\"^^xsd:dateTime")
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_chrono_naive_write_trig() {
        let naive = chrono::NaiveDateTime::parse_from_str(
            "2026-03-02T09:00:00.00000", "%Y-%m-%dT%H:%M:%S%.f"
        ).unwrap();
        let naive_node = LiteralNode::from(naive);

        let mut naive_buf = vec![];
        naive_node.write_trig(&mut naive_buf).unwrap();
        let naive_string = String::from_utf8(naive_buf).unwrap();
        println!("{naive_string}");
        assert_eq!(
            naive_string,
            String::from("\"2026-03-02T09:00:00\"^^xsd:dateTime")
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_chrono_datetime_offset_write_trig() {
        use chrono::TimeZone;

        let offset = chrono::FixedOffset::east_opt(5 * 3600).unwrap()
            .with_ymd_and_hms(2026, 03, 02, 09, 0, 0).unwrap();
        let offset_node = LiteralNode::try_from(offset).unwrap();

        let mut offset_buf = vec![];
        offset_node.write_trig(&mut offset_buf).unwrap();
        let offset_string = String::from_utf8(offset_buf).unwrap();
        assert_eq!(
            offset_string,
            String::from("\"2026-03-02T09:00:00+05:00\"^^xsd:dateTime")
        );
    }

    #[test]
    fn test_decimal_from_int_string() {
        assert!(LiteralNode::decimal("69").is_ok());
    }

    #[test]
    fn test_decimal_from_decimal_string() {
        assert!(LiteralNode::decimal("69.420").is_ok());
    }

    #[test]
    fn test_decimal_from_signed_int_string() {
        assert!(LiteralNode::decimal("+24").is_ok());
    }

    #[test]
    fn test_decimal_from_signed_decimal_string() {
        assert!(LiteralNode::decimal("-2.468").is_ok());
    }

    #[test]
    fn test_decimal_from_f32() {
        assert_eq!(
            LiteralNode::Decimal(Cow::Owned(String::from("69.42"))),
            LiteralNode::from(69.420f32)
        );
    }
    
    #[test]
    fn test_valid_gyear_signed() {
        assert!(LiteralNode::gyear("-0099").is_ok());
    }

    #[test]
    fn test_valid_gyear_really_old() {
        assert!(LiteralNode::gyear("-4206969").is_ok());
    }

    #[test]
    fn test_valid_gyear_far_future() {
        assert!(LiteralNode::gyear("696969").is_ok());
    }

    #[test]
    fn test_valid_gyear_unsigned() {
        assert!(LiteralNode::gyear("1999").is_ok());
    }

    #[test]
    fn test_valid_gyear_utc() {
        assert!(LiteralNode::gyear("-0069Z").is_ok());
    }

    #[test]
    fn test_valid_gyear_offset() {
        assert!(LiteralNode::gyear("-0069+12:00").is_ok());
    }

    #[test]
    fn test_valid_gyear_invalid_offset() {
        assert!(LiteralNode::gyear("-0069+12:00:59").is_err());
    }

    #[test]
    fn test_invalid_gyear_too_short() {
        assert!(LiteralNode::gyear("-420").is_err());
    }

    #[test]
    fn test_invalid_gyear_too_short_unsigned() {
        assert!(LiteralNode::gyear("69").is_err());
    }

    #[test]
    fn test_valid_gyear_from_i32_unsigned_write_trig() {
        let gyear_node = LiteralNode::gyear_from_i32(69);

        let mut buf = vec![];
        gyear_node.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("\"0069\"^^xsd:gYear")
        );
    }

    #[test]
    fn test_valid_gyear_from_i32_signed_write_trig() {
        let gyear_node = LiteralNode::gyear_from_i32(-420);

        let mut buf = vec![];
        gyear_node.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("\"-0420\"^^xsd:gYear")
        );
    }

    #[test]
    fn test_valid_language_string() {
        assert!(LiteralNode::string(
            Some("fr"), String::from("oui")).is_ok()
        )
    }

    #[test]
    fn test_invalid_language_string() {
        assert!(LiteralNode::string(
            Some("french"), String::from("non")).is_ok()
        )
    }

    #[test]
    fn test_no_language_string() {
        assert!(LiteralNode::string(
            None::<String>, String::from("random string")).is_ok()
        )
    }

    #[test]
    fn test_no_language_string_write_trig() {
        let node = LiteralNode::string_no_lang("My Literal");

        let mut buf = vec![];
        node.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("\"My Literal\"")
        );
    }

    #[test]
    fn test_en_language_string_write_trig() {
        let node = LiteralNode::string_en("My Literal");

        let mut buf = vec![];
        node.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("\"My Literal\"@en")
        );
    }

    #[test]
    fn test_custom_language_string_write_trig() {
        let node = LiteralNode::string(
            Some("eng"), "My Literal"
        ).unwrap();

        let mut buf = vec![];
        node.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("\"My Literal\"@eng")
        );
    }

    #[test]
    fn test_blank_node_write_trig() {
        let blank = BlankNode::new("myprefix");

        let mut buf = vec![];
        blank.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("_:myprefix")
        )
    }

    #[test]
    fn test_blank_node_write_trig_w_escape_char() {
        let blank = BlankNode::new("my_prefix");

        let mut buf = vec![];
        blank.write_trig(&mut buf).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("_:my\\_prefix")
        )
    }
}