use std::borrow::Cow;

use crate::raw::RawNode;

/// A [`LangString`] is an `xsd:String` type, which includes a language tag.
/// 
/// In RDF formats, the language tag is typically provided following a string, 
/// prepended with *@* (e.g. `"Some Text"@en`)
#[derive(Debug)]
pub struct LangString {
    value: Cow<'static, str>,
    language: Cow<'static, str>
}

impl LangString {
    /// Create a new [`LangString`].
    pub fn new<C: Into<Cow<'static, str>>>(value: C, language: C) -> LangString {
        LangString {
            value: value.into(),
            language: language.into()
        }
    }

    /// Create a new [`LangString`] with `language` set to "en".
    pub fn new_en<C: Into<Cow<'static, str>>>(value: C) -> LangString {
        LangString {
            value: value.into(),
            language: Cow::Borrowed("en")
        }
    }

    /// Get a reference to the `value`.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get a reference to the `language`.
    pub fn language(&self) -> &str {
        &self.language
    }
}

/// A [`Literal`] is a list of possible __xsd__ types.
/// 
/// It implements [`Into<RawNode>`] to be passed quickly to [`RawTriple`]s.
#[derive(Debug)]
pub enum Literal {
    /// A Rust native `str` type.
    String(Cow<'static, str>),
    /// A Rust native `str` type, with a `language` tag.
    LangString(LangString)
}

impl Literal {
    /// Create a new [`Literal::String`].
    pub fn string<C: Into<Cow<'static, str>>>(value: C) -> Literal {
        Literal::String(value.into())
    }

    /// Create a new [`Literal::LangString`].
    pub fn lang_string<C: Into<Cow<'static, str>>>(
        value: C, language: C
    ) -> Literal {
        Literal::LangString(LangString::new(value, language))
    }

    /// Create a new [`Literal::LangString`] with the language already set to "en".
    pub fn lang_string_en<C: Into<Cow<'static, str>>>(value: C) -> Literal {
        Literal::LangString(LangString::new_en(value))
    }
}

impl Into<RawNode> for Literal {
    fn into(self) -> RawNode {
        RawNode::Literal(self)
    }
}