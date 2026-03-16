#[derive(Clone, Debug)]
pub enum RdfTrigError {
    InvalidBoolean(String),
    InvalidDateTime(String),
    InvalidDecimal(String),
    InvalidGYear(String),
    InvalidIri(String),
    InvalidLanguage(String)
}

impl std::fmt::Display for RdfTrigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfTrigError::InvalidBoolean(bool) => {
                write!(f, "{} is an invalid boolean", bool)
            },
            RdfTrigError::InvalidDateTime(dt) => {
                write!(f, "{} is an invalid dateTime", dt)
            },
            RdfTrigError::InvalidDecimal(dec) => {
                write!(f, "{} is an invalid decimal", dec)
            },
            RdfTrigError::InvalidGYear(gy) => {
                write!(f, "{} is an invalid gYear", gy)
            },
            RdfTrigError::InvalidIri(lang) => {
                write!(f, "{} is an IRI component", lang)
            },
            RdfTrigError::InvalidLanguage(lang) => {
                write!(f, "{} is an invalid language code", lang)
            }
        }
    }
}

impl std::error::Error for RdfTrigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Update if any external errors are required.
        None
    }
}