use std::convert::TryFrom;
use std::time;

use crate::id::Id;
use crate::line::Line;
use crate::word::Word;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: Id,
    pub reply_on: Option<Id>,
    pub roomname: Word,
    pub username: Word,
    pub timestamp: time::SystemTime,
    pub message: Line,
}

impl Message {
    pub(super) fn try_parse(words: &[&str]) -> Result<Self, &'static str> {
        macro_rules! parse_i64 {
            ($val:expr, $field:expr) => {
                match $val.parse::<i64>() {
                    Err(_) => {
                        return Err(concat!("got invalid invalid value for message ", $field))
                    }
                    Ok(i) => i,
                }
            };
        }
        macro_rules! parse_id {
            ($val:expr, $field:expr) => {
                match Id::try_from($val) {
                    Err(_) => {
                        return Err(concat!("got invalid invalid value for message ", $field))
                    }
                    Ok(i) => i,
                }
            };
        }

        let id = parse_id!(parse_i64!(words[3], "id"), "id");
        let reply_on = match parse_i64!(words[4], "reply_on") {
            -1 => None,
            id => Some(parse_id!(id, "reply_on")),
        };
        let roomname = Word::try_from(words[0].to_string())?;
        let username = Word::try_from(words[1].to_string())?;

        let timestamp = parse_i64!(words[2], "timestamp") as u64;
        let timestamp = time::UNIX_EPOCH + time::Duration::from_micros(timestamp);

        let message = words[5..].join(" ");
        let message = Line::try_from(message)?;

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
