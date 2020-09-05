use crate::message::Message;
use crate::util::{expect_word, parsei64};
use crate::word::Word;

#[derive(Debug)]
pub enum PushMessage {
    Online(i64, Box<Word>), // i64, word
    Message(Message),
    Invite(Box<Word>, Box<Word>), // word, word
    Join(Box<Word>, Box<Word>),   // word, word
}

impl PushMessage {
    pub(super) fn parse(s: &str) -> Option<Self> {
        let words: Vec<_> = s.split(' ').collect();
        assert!(words[0] == "_push");
        let item = match words[1] {
            "online" => PushMessage::Online(parsei64(&words[2]), expect_word(words[3])),
            "message" => {
                let message = Message::try_parse(&words[2..]).unwrap();
                PushMessage::Message(message)
            }
            "invite" => PushMessage::Invite(expect_word(words[2]), expect_word(words[3])),
            "join" => PushMessage::Join(expect_word(words[2]), expect_word(words[3])),

            // we can ignore this
            "ping" => return None,

            _ => panic!("unknown push type"),
        };

        Some(item)
    }
}
