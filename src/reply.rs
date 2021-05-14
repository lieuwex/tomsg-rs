use std::convert::TryFrom;

use super::line::Line;
use super::message::Message;
use super::word::Word;
use crate::util::{expect_word, parsei64};

pub(super) enum InternalReply {
    Normal(Reply),
    HistoryInit(i64),
    HistoryMessage(i64, Message), // index, message
}

/// A reply type and related information.
///
/// For every possible type there is a function defined which converts this `Reply` into an
/// `Option`, if the `Reply` is that particular variant `Some(val)` is returned. Otherwise, `None`
/// is returned.
/// This is useful for quickly extracting the wanted response value.
#[derive(Debug, Clone)]
pub enum Reply {
    /// Represents a succesful processing of a sent `Command`.
    Ok,
    /// A numeric value returned to a sent `Command`.
    Number(i64),
    /// An error value returned to a sent `Command`.
    Error(Line),
    /// A name value returned to a sent `Command`.
    Name(Word),
    /// A list of name values returned to a sent `Command`.
    List(Vec<Word>),
    /// Response to a sent 'Command::Ping`.
    Pong,
    /// Resonse of a list of historical `Message` instances.
    History(Vec<Message>),
    /// A single `Message` instance.
    Message(Message),
}

impl Reply {
    #[must_use]
    pub fn ok(self) -> Option<()> {
        match self {
            Reply::Ok => Some(()),
            _ => None,
        }
    }
    #[must_use]
    pub fn number(self) -> Option<i64> {
        match self {
            Reply::Number(n) => Some(n),
            _ => None,
        }
    }
    #[must_use]
    pub fn error(self) -> Option<Line> {
        match self {
            Reply::Error(e) => Some(e),
            _ => None,
        }
    }
    #[must_use]
    pub fn name(self) -> Option<Word> {
        match self {
            Reply::Name(n) => Some(n),
            _ => None,
        }
    }
    #[must_use]
    pub fn list(self) -> Option<Vec<Word>> {
        match self {
            Reply::List(l) => Some(l),
            _ => None,
        }
    }
    #[must_use]
    pub fn pong(self) -> Option<()> {
        match self {
            Reply::Pong => Some(()),
            _ => None,
        }
    }
    #[must_use]
    pub fn history(self) -> Option<Vec<Message>> {
        match self {
            Reply::History(h) => Some(h),
            _ => None,
        }
    }
    #[must_use]
    pub fn message(self) -> Option<Message> {
        match self {
            Reply::Message(m) => Some(m),
            _ => None,
        }
    }
}

/// returns the tag and the InternalReply
pub(super) fn parse(s: &str) -> (Word, InternalReply) {
    let words: Vec<_> = s.split(' ').collect();

    let make = |command: InternalReply| -> (Word, InternalReply) {
        let tag = Word::try_from(words[0].to_string()).unwrap();
        (tag, command)
    };
    let make_normal =
        |command: Reply| -> (Word, InternalReply) { make(InternalReply::Normal(command)) };

    let expect_line = |s: String| Line::try_from(s).unwrap();

    match words[1] {
        "ok" => make_normal(Reply::Ok),
        "number" => make_normal(Reply::Number(parsei64(words[2]))),
        "error" => make_normal(Reply::Error(expect_line(words[2..].join(" ")))),
        "name" => make_normal(Reply::Name(expect_word(words[2]))),
        "list" => make_normal(Reply::List(words[3..].iter().map(expect_word).collect())),
        "pong" => make_normal(Reply::Pong),
        "message" => {
            let message = Message::try_parse(&words[2..]).unwrap();
            make_normal(Reply::Message(message))
        }

        // still needs to be handled
        "history" => make(InternalReply::HistoryInit(parsei64(words[2]))),
        "history_message" => {
            let index = parsei64(words[2]);
            let message = Message::try_parse(&words[3..]).unwrap();
            make(InternalReply::HistoryMessage(index, message))
        }

        w => panic!("unexpected response type: '{}'", w),
    }
}
