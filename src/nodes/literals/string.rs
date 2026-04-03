use std::borrow::Cow;
use std::io::{self, Write};

use crate::errors::RdfTrigError;
use crate::traits::{ToStatic, WriteNQuads, WriteTriG};
#[cfg(feature = "tokio")]
use crate::traits::{WriteNQuadsAsync, WriteTriGAsync};
use crate::utils::write_escaped_literal;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StringLiteral<'a> {
    value: Cow<'a, str>,
    language: Option<Cow<'a, str>>
}

impl<'a> StringLiteral<'a> {
    /// Create a new `LiteralNode::LangString` from the provided `value` and 
    /// `language` `str`s.
    /// 
    /// Validates the `language` as an ISO 639-1 2-digit language code.
    pub fn new<V, L>(value: V, language: Option<L>)
    -> Result<StringLiteral<'a>, RdfTrigError>
    where
        V: Into<Cow<'a, str>>,
        L: Into<Cow<'a, str>>
    {
        let language = language.and_then(|lang| Some(lang.into()));

        if let Some(lang) = &language {
            if !(lang.len() == 2 || lang.len() == 3) ||
            !lang.chars().all(char::is_alphabetic) {
                return Err(RdfTrigError::InvalidLanguage(lang.to_string()))
            }
        }


        Ok(StringLiteral { value: value.into(), language })
    }

    /// Create a new `LiteralNode::LangString` from the provided `value`.
    /// 
    /// Sets the `language` to "en" (English).
    pub fn new_en<V: Into<Cow<'a, str>>>(value: V)
    -> StringLiteral<'a> {
        StringLiteral { value: value.into(), language: Some("en".into()) }
    }

    /// Get a `str` slice reference to the `value`.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get a `str` slice reference to the `language`.
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }
}

impl<'a> ToStatic for StringLiteral<'a> {
    type StaticType = StringLiteral<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        let language = self.language.as_ref().and_then(|lang| {
            Some(Cow::Owned(lang.clone().into_owned()))
        });      

        StringLiteral {
            value: Cow::Owned(self.value.clone().into_owned()),
            language
        }
    }
}

impl<'a> WriteNQuads for StringLiteral<'a> {
    fn write_nquads<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"\"")?;
        write_escaped_literal(writer, &self.value)?;
        writer.write_all(b"\"")?;

        if let Some(lang) = self.language() {
            writer.write_all(b"@")?;
            writer.write_all(lang.as_bytes())?;
        }   

        Ok(())
    }
}

impl<'a> WriteTriG for StringLiteral<'a> {
    #[inline]
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // Representation of StringLiterals in TriG and N-Quads is identical.
        self.write_nquads(writer)
    }
}