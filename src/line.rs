use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt;
use std::mem;

/// A `Line` is a `str` which does not contain newlines.
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
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line(str);

impl Line {
    #[allow(clippy::transmute_ptr_to_ptr)]
    fn from_borrowed(s: &str) -> &Self {
        unsafe { mem::transmute(s) }
    }

    fn from_owned(s: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(s) }
    }

    fn into_owned(self: Box<Self>) -> Box<str> {
        unsafe { mem::transmute(self) }
    }

    /// Extracts a string slice containing the contents of the `Line`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts this `Box<Line>` into a `String`.
    pub fn into_string(self: Box<Self>) -> String {
        self.into_owned().into_string()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Clone for Box<Line> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl ToOwned for Line {
    type Owned = Box<Line>;

    fn to_owned(&self) -> Self::Owned {
        Self::from_owned(self.0.to_owned().into_boxed_str())
    }
}

impl TryFrom<String> for Box<Line> {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.contains('\n') {
            Err("string contains newlines")
        } else {
            Ok(Line::from_owned(val.into_boxed_str()))
        }
    }
}

impl<'a> TryFrom<&'a str> for &'a Line {
    type Error = &'static str;

    fn try_from(val: &'a str) -> Result<Self, Self::Error> {
        if val.contains('\n') {
            Err("string contains newlines")
        } else {
            Ok(Line::from_borrowed(val))
        }
    }
}

impl Borrow<str> for Line {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
