use std::convert::TryFrom;

use crate::word::Word;

pub fn parsei64(item: &str) -> i64 {
    item.parse::<i64>().unwrap()
}

pub fn expect_word<S: ToString>(s: S) -> Word {
    Word::try_from(s.to_string()).unwrap()
}
