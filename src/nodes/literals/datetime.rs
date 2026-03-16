use std::borrow::Cow;
use std::io::{self, Write};

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};
use time::{OffsetDateTime, PrimitiveDateTime};
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;

use crate::errors::RdfTrigError;
use crate::nodes::object::Object;
use crate::nodes::literals::LiteralNode;
use crate::traits::{ToStatic, WriteTriG};

const FMT_NAIVE_SUBSECOND: &[time::format_description::FormatItem<'_>] = 
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]");
const FMT_NAIVE_ISO: &[time::format_description::FormatItem<'_>] = 
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

/// A wrapper around a [`Cow<'a, str>`], which can be constructed from a `str` 
/// or [`From`] one of the following types:
///  - [`chrono::DateTime`],
///  - [`chrono::NaiveDateTime`],
///  - [`time::OffsetDateTime`],
///  - [`time::PrimitiveDateTime`]
/// 
/// This crate uses `time` to validate XML Schema `dateTime`s, but stores the 
/// values as `Cow`s to avoid excessive parsing then formatting.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DateTimeLiteral<'a>(pub(crate) Cow<'a, str>);

impl<'a> DateTimeLiteral<'a> {
    /// Declare a `LiteralNode::Datetime` from the provided value.
    /// 
    /// Returns an `RdfTrigError::InvalidDateTime` if the provided value cannot 
    /// be parsed as an XML Schema `dateTime` ("1900-01-01T00:00:00.000", with 
    /// or without "Z" or a timezone offset).
    // Custom function instead of TryFrom to allow Into<Cow...> values.
    pub fn try_from_str<C: Into<Cow<'a, str>>>(value: C)
    -> Result<DateTimeLiteral<'a>, RdfTrigError> {
        let value = value.into();

        if OffsetDateTime::parse(&value, &Rfc3339).is_ok() {
            return Ok(DateTimeLiteral(value));
        }

        if PrimitiveDateTime::parse(
            &value, &FMT_NAIVE_SUBSECOND
        ).is_ok() || PrimitiveDateTime::parse(
            &value, &FMT_NAIVE_ISO
        ).is_ok() {
            Ok(DateTimeLiteral(value))
        } else {
            Err(RdfTrigError::InvalidDateTime(value.to_string()))
        }
    }
}

#[cfg(feature = "chrono")]
impl<'a> From<NaiveDateTime> for DateTimeLiteral<'a> {
    fn from(value: NaiveDateTime) -> Self {
        DateTimeLiteral(
            Cow::Owned(value.format("%Y-%m-%dT%H:%M:%S").to_string())
        )
    }
}

#[cfg(feature = "chrono")]
impl<'a> From<DateTime<FixedOffset>> for DateTimeLiteral<'a> {
    fn from(value: DateTime<FixedOffset>) -> Self {
        DateTimeLiteral(
            Cow::Owned(value.format("%+").to_string())
        )
    }
}

#[cfg(feature = "chrono")]
impl<'a> From<DateTime<Local>> for DateTimeLiteral<'a> {
    fn from(value: DateTime<Local>) -> Self {
        DateTimeLiteral(Cow::Owned(value.format("%+").to_string()))
    }
}

#[cfg(feature = "chrono")]
impl<'a> From<DateTime<Utc>> for DateTimeLiteral<'a> {
    fn from(value: DateTime<Utc>) -> Self {
        DateTimeLiteral(Cow::Owned(value.format("%+").to_string()))
    }
}

#[cfg(feature = "time")]
impl<'a> TryFrom<PrimitiveDateTime> for DateTimeLiteral<'a> {
    type Error = RdfTrigError;

    fn try_from(value: PrimitiveDateTime) -> Result<Self, Self::Error> {
        let fmt = format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second]"
        );

        match value.format(&fmt) {
            Ok(dt_str) => Ok(DateTimeLiteral(Cow::Owned(dt_str))),
            Err(_) => {
                Err(RdfTrigError::InvalidDateTime(
                        "Invalid PrimitiveDateTime".to_owned()
                ))
            }
        }
    }
}

#[cfg(feature = "time")]
impl<'a> TryFrom<OffsetDateTime> for DateTimeLiteral<'a> {
    type Error = RdfTrigError;

    fn try_from(value: OffsetDateTime) -> Result<Self, Self::Error> {
        match value.format(&Rfc3339) {
            Ok(dt_str) => Ok(DateTimeLiteral(Cow::Owned(dt_str))),
            Err(_) => {
                Err(RdfTrigError::InvalidDateTime(
                        "Invalid OffsetDateTime".to_owned()
                ))
            }
        }
    }
}

impl<'a> Into<LiteralNode<'a>> for DateTimeLiteral<'a> {
    #[inline(always)]
    fn into(self) -> LiteralNode<'a> {
        LiteralNode::DateTime(self)
    }
}

impl<'a> Into<Object<'a>> for DateTimeLiteral<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Literal(self.into())
    }
}

impl<'a> ToStatic for DateTimeLiteral<'a> {
    type StaticType = DateTimeLiteral<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        DateTimeLiteral(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl WriteTriG for DateTimeLiteral<'_> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"\"")?;
        writer.write_all(self.0.as_bytes())?;
        writer.write_all(b"\"^^xsd:dateTime")?;

        Ok(())
    }
}