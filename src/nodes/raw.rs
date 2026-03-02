use std::borrow::Cow;
use std::io::{Result as IoResult, Write};

use time::{OffsetDateTime, PrimitiveDateTime};
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};

use crate::errors::RdfLiteError;
use crate::namespaces::{Namespace, NamespaceId};
use crate::traits::WriteTriG;

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
        writer.write_all(self.0.as_bytes())
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
    /// Returns an `RdfLiteError::InvalidBoolean` if the provided value cannot 
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
    -> Result<LiteralNode, RdfLiteError> {
        let cow_val: Cow<'static, str> = value.into();

        match &*cow_val {
            "true" | "1" => Ok(LiteralNode::Boolean(true)),
            "false" | "0" => Ok(LiteralNode::Boolean(false)),
            _ => Err(RdfLiteError::InvalidBoolean(cow_val))
        }        
    }

    /// Declare a `LiteralNode::Datetime` from the provided value.
    /// 
    /// Returns an `RdfLiteError::InvalidDateTime` if the provided value cannot 
    /// be parsed as an XML Schema `dateTime` ("1900-01-01T00:00:00.000", with 
    /// or without "Z" or a timezone offset).
    /// 
    /// This crate uses `time` to validate XSD `dateTime`s, but implements 
    /// [`From<>`] for [`chrono::DateTime`], [`chrono::NaiveDateTime`], 
    /// [`time::OffsetDateTime`] and [`time::PrimitiveDateTime`] with the 
    /// relevant `chrono` or `time` feature flags enabled.
    pub(crate) fn datetime<C: Into<Cow<'static, str>>>(value: C)
    -> Result<LiteralNode, RdfLiteError> {
        let cow_val: Cow<'static, str> = value.into();

        if OffsetDateTime::parse(&cow_val, &Rfc3339).is_ok() {
            return Ok(LiteralNode::Datetime(cow_val));
        }

        let is_naive = PrimitiveDateTime::parse(
            &cow_val, &FMT_NAIVE_SUBSECOND
        ).is_ok() || PrimitiveDateTime::parse(
            &cow_val, &FMT_NAIVE_ISO
        ).is_ok();

        if is_naive {
            Ok(LiteralNode::Datetime(cow_val))
        } else {
            Err(RdfLiteError::InvalidDateTime(cow_val))
        }
    }

    /// Declare a `LiteralNode::Decimal` type from the provided value.
    /// 
    /// Returns an `RdfLiteError::InvalidDecimal` if the provided value cannot 
    /// be parsed as an `f32`.
    /// 
    /// For ease, `LiteralNode` also implements [`From<f32>`] for quick 
    /// conversions.
    pub(crate) fn decimal<C: Into<Cow<'static, str>>>(value: C)
    -> Result<LiteralNode, RdfLiteError> {
        // Deliberately does not drop the `str` in place of the f32 at any 
        // point, as the crate would only have to return it to that format for 
        // io::Write.
        let cow_val: Cow<'static, str> = value.into();

        match cow_val.parse::<f32>() {
            Ok(_) => Ok(LiteralNode::Decimal(cow_val)),
            Err(_) => Err(RdfLiteError::InvalidDecimal(cow_val))
        }
    }

    /// Declare a `LiteralNode::GYear` type from the provided value.
    /// 
    /// Returns an `RdfLiteError::InvalidGYear` if the provided value cannot be 
    /// parsed as an XSD gYear (CE/BCE year, with or without a timezone offset).
    pub(crate) fn gyear<C: Into<Cow<'static, str>>>(value: C)
    -> Result<LiteralNode, RdfLiteError> {
        let valid_formats = ["%Y", "%Y%:z"];

        let cow_val: Cow<'static, str> = value.into();

        // for fmt in valid_formats {
        //     if chrono::DateTime::parse_from_str(&cow_val, fmt).is_ok() {
        //         return Ok(LiteralNode::GYear(cow_val))
        //     }
        // }

        Err(RdfLiteError::InvalidGYear(cow_val))
    }        

    /// Create a new `LiteralNode::String` with the provided `language` and 
    /// string `value`.
    pub(crate) fn string<L, V>(language: Option<L>, value: V) -> LiteralNode 
    where
        L: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>
    {
        LiteralNode::String(StringLiteral::new(language, value))
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
    type Error = RdfLiteError;

    fn try_from(value: PrimitiveDateTime) -> Result<Self, Self::Error> {
        match value.format(&Rfc3339) {
            Ok(dt_str) => Ok(LiteralNode::Datetime(Cow::Owned(dt_str))),
            Err(_) => {
                Err(RdfLiteError::InvalidDateTime(
                        "Invalid PrimitiveDateTime".into()
                ))
            }
        }
    }
}

#[cfg(feature = "time")]
impl TryFrom<OffsetDateTime> for LiteralNode {
    type Error = RdfLiteError;

    fn try_from(value: OffsetDateTime) -> Result<Self, Self::Error> {
        match value.format(&Rfc3339) {
            Ok(dt_str) => Ok(LiteralNode::Datetime(Cow::Owned(dt_str))),
            Err(_) => {
                Err(RdfLiteError::InvalidDateTime(
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
pub struct StringLiteral {
    language: Option<Cow<'static, str>>,
    value: Cow<'static, str>
}

impl StringLiteral {
    /// Create a new `StringLiteral` from a `language` tag and `value`.
    pub(crate) fn new<L, V>(language: Option<L>, value: V) -> StringLiteral 
    where
        L: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>
    {
        StringLiteral {
            language: language.map(|l| l.into()),
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
    fn test_bools() {
        let good_true_one = "true";
        assert!(LiteralNode::boolean(good_true_one).is_ok());
        let good_true_two = String::from("1");
        assert!(LiteralNode::boolean(good_true_two).is_ok());

        let bad_true_one = String::from("True");
        assert!(LiteralNode::boolean(bad_true_one).is_err());
        let bad_true_two = "A completely random str.";
        assert!(LiteralNode::boolean(bad_true_two).is_err());

        let raw_true = true;
        let raw_true_built = LiteralNode::from(raw_true);
        assert_eq!(raw_true_built, LiteralNode::Boolean(true));

        let mut buf: Vec<u8> = vec![];
        raw_true_built.write_trig(&mut buf).unwrap();

        let as_string = String::from_utf8(buf).unwrap();
        assert_eq!(as_string, String::from("true"));

        let good_false_one = "false";
        assert!(LiteralNode::boolean(good_false_one).is_ok());
        let good_false_two = String::from("0");
        assert!(LiteralNode::boolean(good_false_two).is_ok());

        let bad_false_one = String::from("False");
        assert!(LiteralNode::boolean(bad_false_one).is_err());
        let bad_false_two = "A completely random str.";
        assert!(LiteralNode::boolean(bad_false_two).is_err());

        let raw_false = false;
        let raw_false_built = LiteralNode::from(raw_false);
        assert_eq!(raw_false_built, LiteralNode::Boolean(false));

        let mut buf: Vec<u8> = vec![];
        raw_false_built.write_trig(&mut buf).unwrap();

        let as_string = String::from_utf8(buf).unwrap();
        assert_eq!(as_string, String::from("false"));
    }

    #[test]
    fn test_datetimes() {
        let with_utc = "2026-03-02T09:00:00.000Z";
        assert!(LiteralNode::datetime(with_utc).is_ok());

        let with_utc_no_secs = String::from("2026-03-02T09:00:00Z");
        assert!(LiteralNode::datetime(with_utc_no_secs).is_ok());

        let with_tz = "2026-03-02T09:00:00.000+01:00";
        assert!(LiteralNode::datetime(with_tz).is_ok());

        let without_tz_or_utc = "2026-03-02T09:00:00";
        assert!(LiteralNode::datetime(without_tz_or_utc).is_ok());

        let invalid = String::from("Random string");
        assert!(LiteralNode::datetime(invalid).is_err())
    }
}