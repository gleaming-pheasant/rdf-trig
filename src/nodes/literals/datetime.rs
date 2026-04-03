use std::borrow::Cow;

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};
use time::{OffsetDateTime, PrimitiveDateTime};
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
#[cfg(feature = "tokio")]
use tokio::io::AsyncWriteExt;

use crate::errors::RdfTrigError;
use crate::traits::{ToStatic, WriteNQuads, WriteTriG};
#[cfg(feature = "tokio")]
use crate::traits::{WriteNQuadsAsync, WriteTriGAsync};

macro_rules! impl_write_trig_sync {
    (
        $target:ty, // The type to implement the function for.
        $( $lt:lifetime )?, // Optional lifetime if the target has one.
        $( $this:ident )?, // Optional declaration which maps `this` to `self`.
        [ $($part:expr),* $(,)? ], // Expressions to write (e.g. b"^^xsd:decimal").
        // $(< $($implementor:ident),* $(,)? >)? // List of implementors which will be called after parts (e.g. b for a captured BooleanLiteral).
    ) => {
        impl $( <$lt> )? WriteTriG for $target {
            fn write_trig<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                $( let $this = self; )?

                $(
                    writer.write_all($part)?;
                )*
                // $(
                //     $implementor.write_trig(writer)?;
                // )*
                Ok(())
            }
        }
    };
}

const XSD_DATETIME_IRI: &'static str = "<http://www.w3.org/2001/XMLSchema#dateTime>";

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

impl<'a> ToStatic for DateTimeLiteral<'a> {
    type StaticType = DateTimeLiteral<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        DateTimeLiteral(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'a> WriteNQuads for DateTimeLiteral<'a> {
    fn write_nquads<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(b"\"")?;
        writer.write_all(self.0.as_bytes())?;
        writer.write_all(b"\"^^")?;
        writer.write_all(XSD_DATETIME_IRI.as_bytes())?;
        Ok(())
    }
}

impl_write_trig_sync!(DateTimeLiteral<'a>);

// impl<'a> WriteTriG for DateTimeLiteral<'a> {
//     fn write_trig<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
//         writer.write_all(b"\"")?;
//         writer.write_all(self.0.as_bytes())?;
//         writer.write_all(b"\"^^xsd:dateTime")?;
//         Ok(())
//     }
// }

#[cfg(feature = "tokio")]
impl<'a> WriteNQuadsAsync for DateTimeLiteral<'a> {
    fn write_nquads_async<W>(
        &self, writer: &mut W
    ) -> impl Future<Output = std::io::Result<()>> + Send
    where
        W: tokio::io::AsyncWrite + Unpin + Send
    {
        async move {
            writer.write_all(b"\"").await?;
            writer.write_all(self.0.as_bytes()).await?;
            writer.write_all(b"\"^^").await?;
            writer.write_all(XSD_DATETIME_IRI.as_bytes()).await?;
            Ok(())
        }        
    }
}

#[cfg(feature = "tokio")]
impl<'a> WriteTriGAsync for DateTimeLiteral<'a> {
    fn write_trig_async<W>(
        &self, writer: &mut W
    ) -> impl Future<Output = std::io::Result<()>> + Send
    where
        W: tokio::io::AsyncWrite + Unpin + Send
    {
        async move {
            writer.write_all(b"\"").await?;
            writer.write_all(self.0.as_bytes()).await?;
            writer.write_all(b"\"^^xsd:dateTime").await?;
            Ok(())
        }        
    }
}