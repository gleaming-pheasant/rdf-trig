use std::borrow::Cow;
use std::io::{self, Write};

use crate::WriteTriG;
use crate::nodes::object::Object;
use crate::nodes::literals::LiteralNode;
use crate::errors::RdfTrigError;
use crate::traits::ToStatic;
use crate::utils::write_escaped_literal;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LangStringLiteral<'a> {
    value: Cow<'a, str>,
    language: Cow<'a, str>
}

impl<'a> LangStringLiteral<'a> {
    /// Create a new `LiteralNode::LangString` from the provided `value` and 
    /// `language` `str`s.
    /// 
    /// Validates the `language` as an ISO 639-1 2-digit language code.
    pub fn new<V, L>(value: V, language: L)
    -> Result<LangStringLiteral<'a>, RdfTrigError>
    where
        V: Into<Cow<'a, str>>,
        L: Into<Cow<'a, str>>
    {
        let language = language.into();

        if !(language.len() == 2 || language.len() == 3) ||
        !language.chars().all(char::is_alphabetic) {
            return Err(RdfTrigError::InvalidLanguage(language.to_string()))
        }

        Ok(LangStringLiteral { value: value.into(), language })
    }

    /// Create a new `LiteralNode::LangString` from the provided `value`.
    /// 
    /// Sets the `language` to "en" (English).
    pub fn new_en<V: Into<Cow<'a, str>>>(value: V)
    -> LangStringLiteral<'a> {
        LangStringLiteral { value: value.into(), language: "en".into() }
    }

    /// Get a `str` slice reference to the `value`.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get a `str` slice reference to the `language`.
    pub fn language(&self) -> &str {
        &self.language
    }
}

impl<'a> Into<LiteralNode<'a>> for LangStringLiteral<'a> {
    #[inline(always)]
    fn into(self) -> LiteralNode<'a> {
        LiteralNode::LangString(self)
    }
}

impl<'a> Into<Object<'a>> for LangStringLiteral<'a> {
    #[inline]
    fn into(self) -> Object<'a> {
        Object::Literal(self.into())
    }
}

impl<'a> ToStatic for LangStringLiteral<'a> {
    type StaticType = LangStringLiteral<'static>;

    #[inline]
    fn to_static(&self) -> Self::StaticType {
        LangStringLiteral {
            value: Cow::Owned(self.value.clone().into_owned()),
            language: Cow::Owned(self.language.clone().into_owned()),
        }
    }
}

impl<'a> WriteTriG for LangStringLiteral<'a> {
    fn write_trig<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"\"")?;
        write_escaped_literal(writer, &self.value)?;
        writer.write_all(b"\"@")?;
        writer.write_all(self.language.as_bytes())?;

        Ok(())
    }
}