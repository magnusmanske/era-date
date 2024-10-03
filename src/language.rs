use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    German,
    English,
}

impl Language {
    pub(crate) fn extension(&self, year: i32) -> &str {
        match self {
            Language::German => ".", // German does not have extensions
            Language::English => match year.abs() % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            },
        }
    }

    pub(crate) fn era(&self, year: i32) -> &str {
        if year < 0 {
            match self {
                Language::German => " v.Chr.",
                Language::English => " BCE",
            }
        } else {
            ""
        }
    }

    pub(crate) fn decade(&self) -> &str {
        match self {
            Language::German => "er",
            Language::English => "s",
        }
    }

    pub(crate) fn century(&self) -> &str {
        match self {
            Language::German => "Jahrhundert",
            Language::English => "century",
        }
    }

    pub(crate) fn millennium(&self) -> &str {
        match self {
            Language::German => "Jahrtausend",
            Language::English => "millennium",
        }
    }
}

/// Sets the language from a string, defaults to English if the language is not supported.
impl<'a> From<&'a str> for Language {
    fn from(s: &'a str) -> Self {
        match s.to_lowercase().trim() {
            "de" => Language::German,
            _ => Language::English, // Fallback, default language
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::German => write!(f, "de"),
            Language::English => write!(f, "en"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_render_language_code() {
        assert_eq!(Language::from("de"), Language::German);
        assert_eq!(Language::from("en"), Language::English);
        assert_eq!(Language::from("foobar"), Language::English);
    }
}
