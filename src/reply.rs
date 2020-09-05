use std::convert::TryInto;

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
#[derive(Debug, Clone)]
pub enum Reply {
    /// Represents a succesful processing of a sent `Command`.
    Ok,
    /// A numeric value returned to a sent `Command`.
    Number(i64),
    /// An error value returned to a sent `Command`.
    Error(Box<Line>),
    /// A name value returned to a sent `Command`.
    Name(Box<Word>),
    /// A list of name values returned to a sent `Command`.
    List(Vec<Box<Word>>),
    /// Response to a sent 'Command::Ping`.
    Pong,
    /// Resonse of a list of historical `Message` instances.
    History(Vec<Message>),
    /// A single `Message` instance.
    Message(Message),
}

/// returns the tag and the InternalReply
pub(super) fn parse(s: &str) -> (Box<Word>, InternalReply) {
    let words: Vec<_> = s.split(' ').collect();

    let make = |command: InternalReply| -> (Box<Word>, InternalReply) {
        let tag: Box<Word> = words[0].to_string().try_into().unwrap();
        (tag, command)
    };
    let make_normal =
        |command: Reply| -> (Box<Word>, InternalReply) { make(InternalReply::Normal(command)) };

    let expect_line = |s: String| -> Box<Line> { s.try_into().unwrap() };

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

        w => panic!(format!("unexpected response type: '{}'", w)),
    }
}
