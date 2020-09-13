use crate::message::Message;
use crate::util::{expect_word, parsei64};
use crate::word::Word;

/// An item pushed from the tomsg server to the client.
#[derive(Debug)]
pub enum PushMessage {
    /// An update to the online state of a person that you participate with in a room.
    Online {
        /// The amount of sessions currently marked as online.
        sessions: i64,
        /// The username of the user.
        username: Word,
    },
    /// A new message is sent in a room that the client participates in.
    Message(Message),
    /// A person invited the current client to a room.
    Invite {
        /// The name of the room the client is invited in.
        roomname: Word,
        /// The username of the user that invited the client.
        inviter: Word,
    },
    /// A person has joined a room you participate in.
    Join {
        /// The room in question.
        roomname: Word,
        /// The username of the user that joined the room.
        username: Word,
    },
}

impl PushMessage {
    pub(super) fn parse(s: &str) -> Option<Self> {
        let words: Vec<_> = s.split(' ').collect();
        assert!(words[0] == "_push");
        let item = match words[1] {
            "online" => PushMessage::Online {
                sessions: parsei64(&words[2]),
                username: expect_word(words[3]),
            },
            "message" => {
                let message = Message::try_parse(&words[2..]).unwrap();
                PushMessage::Message(message)
            }
            "invite" => PushMessage::Invite {
                roomname: expect_word(words[2]),
                inviter: expect_word(words[3]),
            },
            "join" => PushMessage::Join {
                roomname: expect_word(words[2]),
                username: expect_word(words[3]),
            },

            // we can ignore this
            "ping" => return None,

            _ => panic!("unknown push type"),
        };

        Some(item)
    }
}
