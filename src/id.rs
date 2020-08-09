use std::convert::TryFrom;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Id(i64);

impl Id {
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
