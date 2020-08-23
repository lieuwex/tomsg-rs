use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt;

/// A `Line` is a `String` which does not contain newlines.
/// You can obtain a `Line` by calling `try_from` with a `String` argument:
/// ```
/// use tomsg_rs::line::Line;
/// use std::convert::TryFrom;
///
/// let valid = "this is a valid line".to_string();
/// let invalid = "this is not\na valid line".to_string();
///
/// assert!(Line::try_from(valid).is_ok());
/// assert!(Line::try_from(invalid).is_err());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Line(String);

impl Line {
    /// Extracts a string slice containing the contents of the `Line`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts this `Line` into a `String`.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Line {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.contains('\n') {
            Err("string contains newlines")
        } else {
            Ok(Line(val))
        }
    }
}

impl Borrow<str> for Line {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
