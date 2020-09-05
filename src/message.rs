use std::convert::{TryFrom, TryInto};
use std::time;

use crate::id::Id;
use crate::line::Line;
use crate::word::Word;

/// A tomsg message message in a room.
#[derive(Debug, Clone)]
pub struct Message {
    /// The ID of the message.
    pub id: Id,
    /// The ID of the message this message replies on, if any.
    pub reply_on: Option<Id>,
    /// The name of the tomsg room this message is sent in.
    pub roomname: Box<Word>,
    /// The username of the author of this message.
    pub username: Box<Word>,
    /// The time this message was sent.
    pub timestamp: time::SystemTime,
    /// The contents of this message.
    pub message: Box<Line>,
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
        let roomname: Box<Word> = words[0].to_string().try_into()?;
        let username: Box<Word> = words[1].to_string().try_into()?;

        let timestamp = parse_i64!(words[2], "timestamp") as u64;
        let timestamp = time::UNIX_EPOCH + time::Duration::from_micros(timestamp);

        let message = words[5..].join(" ");
        let message: Box<Line> = message.try_into()?;

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
