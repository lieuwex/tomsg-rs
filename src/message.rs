use std::convert::TryFrom;
use std::convert::TryInto;
use std::time;

use crate::id::Id;
use crate::line::Line;
use crate::word::Word;

/// A tomsg message message in a room.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub(super) fn try_parse(words: &[&str]) -> Result<Self, String> {
        let err = |field| format!("got invalid value for message field: {}", field);

        macro_rules! parse {
            ($val:expr, $type:ty, $field:expr) => {
                $val.parse::<$type>().map_err(|_| err($field))?
            };
        }
        let parse_id = |val, field| Id::try_from(val).map_err(|_| err(field));

        let id = parse!(words[3], i64, "id");
        let id = parse_id(id, "id")?;

        let reply_on = match parse!(words[4], i64, "reply_on") {
            -1 => None,
            id => Some(parse_id(id, "reply_on")?),
        };
        let roomname = words[0].to_string().try_into()?;
        let username = words[1].to_string().try_into()?;

        let timestamp = parse!(words[2], u64, "timestamp");
        let timestamp = time::UNIX_EPOCH + time::Duration::from_micros(timestamp);

        let message = words[5..].join(" ");
        let message = message.try_into()?;

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
