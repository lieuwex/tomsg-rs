use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt;
use std::mem;
use std::ops::Deref;

/// A `Word` is a `str` which does not contain spaces or newlines.
///
/// You can obtain a `Word` by calling `try_from`:
/// ```
/// use tomsg_rs::Word;
/// use std::convert::TryFrom;
///
/// let valid = "this_is_a_valid_word";
/// let invalid = "this is not a valid word";
///
/// assert!(<&Word>::try_from(valid).is_ok());
/// assert!(<&Word>::try_from(invalid).is_err());
/// ```
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word(str);

impl Word {
    /// Create an `Word` from the given `val`.
    ///
    /// # Safety
    /// This function is `unsafe` because the `val` is not checked on conformity, only use this
    /// function if you're sure that the given `val` does not contain spaces or newlines.
    #[must_use]
    pub unsafe fn from_str_unchecked(val: &str) -> &Self {
        mem::transmute(val)
    }

    /// Create an `Word` from the given `val`.
    ///
    /// # Safety
    /// This function is `unsafe` because the `val` is not checked on conformity, only use this
    /// function if you're sure that the given `val` does not contain spaces or newlines.
    #[must_use]
    pub unsafe fn from_string_unchecked(val: String) -> Box<Self> {
        let s: Box<str> = val.into_boxed_str();
        mem::transmute(s)
    }

    /// Extracts a string slice containing the contents of the `Word`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts this `Box<Word>` into a `String`.
    #[must_use]
    pub fn into_string(self: Box<Self>) -> String {
        let s: Box<str> = unsafe { mem::transmute(self) };
        String::from(s)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl TryFrom<String> for Box<Word> {
    type Error = &'static str;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.contains(|c| c == '\n' || c == ' ') {
            Err("string contains newlines or spaces")
        } else {
            Ok(unsafe { Word::from_string_unchecked(val) })
        }
    }
}

impl<'a> TryFrom<&'a str> for &'a Word {
    type Error = &'static str;

    fn try_from(val: &'a str) -> Result<Self, Self::Error> {
        if val.contains(|c| c == '\n' || c == ' ') {
            Err("string contains newlines or spaces")
        } else {
            Ok(unsafe { Word::from_str_unchecked(val) })
        }
    }
}

impl Borrow<str> for Word {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Word {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToOwned for Word {
    type Owned = Box<Word>;

    fn to_owned(&self) -> Self::Owned {
        unsafe { Self::from_string_unchecked(self.0.to_owned()) }
    }
}

impl Clone for Box<Word> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}
