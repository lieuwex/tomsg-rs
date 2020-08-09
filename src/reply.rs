use std::convert::TryFrom;

use super::message::Message;
use super::word::Word;
use crate::util::{expect_word, parsei64};

pub(super) enum InternalReplyCommand {
    Normal(ReplyCommand),
    HistoryInit(i64),
    HistoryMessage(i64, Message), // index, message
}

#[derive(Debug)]
pub enum ReplyCommand {
    Ok,                    //
    Number(i64),           // i64
    Error(String),         // string
    Name(Word),            // word
    List(Vec<Word>),       // words
    Pong,                  //
    History(Vec<Message>), //
    Message(Message),      //
}

#[derive(Debug)]
pub struct Reply {
    pub tag: Word,
    pub command: ReplyCommand,
}

pub(super) fn parse(s: &str) -> (Word, InternalReplyCommand) {
    println!("the raw string we got was: '{}'", s);
    let words: Vec<_> = s.split(' ').collect();

    let make = |command: InternalReplyCommand| -> (Word, InternalReplyCommand) {
        let tag = Word::try_from(words[0].to_string()).unwrap();
        (tag, command)
    };
    let make_normal = |command: ReplyCommand| -> (Word, InternalReplyCommand) {
        make(InternalReplyCommand::Normal(command))
    };

    match words[1] {
        "ok" => make_normal(ReplyCommand::Ok),
        "number" => make_normal(ReplyCommand::Number(parsei64(words[2]))),
        "error" => make_normal(ReplyCommand::Error(words[2..].join(" "))),
        "name" => make_normal(ReplyCommand::Name(expect_word(words[2]))),
        "list" => make_normal(ReplyCommand::List(
            words[3..].iter().map(expect_word).collect(),
        )),
        "pong" => make_normal(ReplyCommand::Pong),
        "message" => {
            let message = Message::try_parse(&words[2..]).unwrap();
            make_normal(ReplyCommand::Message(message))
        }

        // still needs to be handled
        "history" => make(InternalReplyCommand::HistoryInit(parsei64(words[2]))),
        "history_message" => {
            let index = parsei64(words[2]);
            let message = Message::try_parse(&words[3..]).unwrap();
            make(InternalReplyCommand::HistoryMessage(index, message))
        }

        w => panic!(format!("unexpected response type: '{}'", w)),
    }
}
