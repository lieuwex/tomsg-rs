use std::convert::TryFrom;
use std::fmt;

/// An `Id` is a non-negative 64-bit integer.
///
/// You can obtain an `Id` by calling `try_from` with a `i64` argument:
/// ```
/// use tomsg_rs::Id;
/// use std::convert::TryFrom;
///
/// let valid: i64 = 0;
/// let invalid: i64 = -1;
///
/// assert!(Id::try_from(valid).is_ok());
/// assert!(Id::try_from(invalid).is_err());
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Id(i64);

impl Id {
    /// Create an `Id` from the given `val`.
    ///
    /// # Safety
    /// This function is `unsafe` because the `val` is not checked on conformity, only use this
    /// function if you're sure that the given `val` is non-negative.
    pub unsafe fn from_i64_unchecked(val: i64) -> Self {
        Id(val)
    }

    /// Extracts the `i64` value from the `Id`.
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl TryFrom<i64> for Id {
    type Error = &'static str;

    fn try_from(val: i64) -> Result<Self, Self::Error> {
        if val < 0 {
            Err("value cannot be negative")
        } else {
            Ok(Id(val))
        }
    }
}

impl Into<i64> for Id {
    fn into(self) -> i64 {
        self.0
    }
}
impl Into<i64> for &Id {
    fn into(self) -> i64 {
        self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
