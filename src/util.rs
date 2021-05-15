use std::convert::TryInto;

use crate::word::Word;

pub fn parsei64(item: &str) -> i64 {
    item.parse::<i64>().unwrap()
}

pub fn expect_word<S: ToString>(s: S) -> Box<Word> {
    s.to_string().try_into().unwrap()
}
