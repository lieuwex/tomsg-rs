use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt;
use std::mem;

/// A `Word` is a `str` which does not contain spaces or newlines.
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word(str);

impl Word {
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

    /// Extracts a string slice containing the contents of the `Word`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts this `Box<Word>` into a `String`.
    pub fn into_string(self: Box<Self>) -> String {
        self.into_owned().into_string()
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Clone for Box<Word> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl ToOwned for Word {
    type Owned = Box<Word>;

    fn to_owned(&self) -> Self::Owned {
        Self::from_owned(self.0.to_owned().into_boxed_str())
    }
}

impl TryFrom<String> for Box<Word> {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.contains(|c| c == '\n' || c == ' ') {
            Err("string contains newlines or spaces")
        } else {
            Ok(Word::from_owned(val.into_boxed_str()))
        }
    }
}

impl<'a> TryFrom<&'a str> for &'a Word {
    type Error = &'static str;

    fn try_from(val: &'a str) -> Result<Self, Self::Error> {
        if val.contains(|c| c == '\n' || c == ' ') {
            Err("string contains newlines or spaces")
        } else {
            Ok(Word::from_borrowed(val))
        }
    }
}

impl Borrow<str> for Word {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
