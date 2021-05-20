use std::borrow::{Borrow, Cow};
use std::convert::TryFrom;
use std::fmt;
use std::mem;
use std::ops::Deref;

/// A `Line` is a `str` which does not contain newlines.
///
/// You can obtain a `Line` by calling `try_from`:
/// ```
/// use tomsg_rs::Line;
/// use std::convert::TryFrom;
///
/// let valid = "this is a valid line";
/// let invalid = "this is not\na valid line";
///
/// assert!(<&Line>::try_from(valid).is_ok());
/// assert!(<&Line>::try_from(invalid).is_err());
/// ```
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line(str);

impl Line {
    /// Create an `Word` from the given `val`.
    ///
    /// # Safety
    /// This function is `unsafe` because the `val` is not checked on conformity, only use this
    /// function if you're sure that the given `val` does not contain spaces or newlines.
    #[must_use]
    pub unsafe fn from_str_unchecked(val: &str) -> &Self {
        mem::transmute(val)
    }

    /// Create an `Line` from the given `val`.
    ///
    /// # Safety
    /// This function is `unsafe` because the `val` is not checked on conformity, only use this
    /// function if you're sure that the given `val` does not contain newlines.
    #[must_use]
    pub unsafe fn from_string_unchecked(val: String) -> Box<Self> {
        let s: Box<str> = val.into_boxed_str();
        mem::transmute(s)
    }

    /// Extracts a string slice containing the contents of the `Line`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts this `Box<Line>` into a `String`.
    #[must_use]
    pub fn into_string(self: Box<Self>) -> String {
        let s: Box<str> = unsafe { mem::transmute(self) };
        String::from(s)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<Box<Line>> for Cow<'_, Line> {
    fn from(line: Box<Line>) -> Self {
        Self::Owned(line)
    }
}
impl<'a> From<&'a Line> for Cow<'a, Line> {
    fn from(line: &'a Line) -> Self {
        Self::Borrowed(line)
    }
}

impl TryFrom<String> for Box<Line> {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.contains('\n') {
            Err("string contains newlines")
        } else {
            Ok(unsafe { Line::from_string_unchecked(val) })
        }
    }
}

impl<'a> TryFrom<&'a str> for &'a Line {
    type Error = &'static str;

    fn try_from(val: &'a str) -> Result<Self, Self::Error> {
        if val.contains('\n') {
            Err("string contains newlines")
        } else {
            Ok(unsafe { Line::from_str_unchecked(val) })
        }
    }
}

impl Borrow<str> for Line {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Line {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToOwned for Line {
    type Owned = Box<Line>;

    fn to_owned(&self) -> Self::Owned {
        unsafe { Self::from_string_unchecked(self.0.to_owned()) }
    }
}

impl Clone for Box<Line> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}
