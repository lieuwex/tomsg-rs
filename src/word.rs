use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt;

/// A `Word` is a `String` which does not contain spaces or newlines.
/// You can obtain a `Word` by calling `try_from` with a `String` argument:
/// ```
/// use tomsg_rs::word::Word;
/// use std::convert::TryFrom;
///
/// let valid = "this_is_a_valid_word".to_string();
/// let invalid = "this is not a valid word".to_string();
///
/// assert!(Word::try_from(valid).is_ok());
/// assert!(Word::try_from(invalid).is_err());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Word(String);

impl Word {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Word {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.contains(|c| c == '\n' || c == ' ') {
            Err("string contains newlines or spaces")
        } else {
            Ok(Word(val))
        }
    }
}

impl Borrow<str> for Word {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
