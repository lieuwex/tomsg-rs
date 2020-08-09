use std::convert::TryFrom;
use std::time;

use super::id::Id;
use super::util::{expect_word, parsei64};
use super::word::Word;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: Id,
    pub reply_on: Option<Id>,
    pub roomname: Word,
    pub username: Word,
    pub timestamp: time::SystemTime,
    pub message: String,
}

impl Message {
    pub(super) fn try_parse(words: &[&str]) -> Result<Self, &'static str> {
        // TODO: better error handling

        let id = Id::try_from(parsei64(words[3])).unwrap();
        let reply_on = match parsei64(words[4]) {
            -1 => None,
            id => Some(Id::try_from(id).unwrap()),
        };
        let roomname = expect_word(words[0]);
        let username = expect_word(words[1]);

        let timestamp = parsei64(words[2]) as u64;
        let timestamp = time::UNIX_EPOCH + time::Duration::from_micros(timestamp);

        let message = words[5..].join(" ");

        Ok(Self {
            id,
            reply_on,
            roomname,
            username,
            timestamp,
            message,
        })
    }
}
