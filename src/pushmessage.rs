use crate::message::Message;
use crate::util::{expect_word, parsei64};
use crate::word::Word;

/// An item pushed from the tomsg server to the client.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PushMessage {
    /// An update to the online state of a person that you participate with in a room.
    Online {
        /// The amount of sessions currently marked as online.
        sessions: i64,
        /// The username of the user.
        username: Box<Word>,
    },
    /// A new message is sent in a room that the client participates in.
    Message(Message),
    /// A person invited the current client to a room.
    ///
    /// If `inviter` is the username of the logged-in user on the current client, it means that
    /// another session of the logged-in user joined the room with name `roomname`.
    Invite {
        /// The name of the room the client is invited in.
        roomname: Box<Word>,
        /// The username of the user that invited the client.
        inviter: Box<Word>,
    },
    /// A person has joined a room you participate in.
    Join {
        /// The room in question.
        roomname: Box<Word>,
        /// The username of the user that joined the room.
        username: Box<Word>,
    },
    /// A person has left a room you participate in.
    ///
    /// If `username` is the username of the logged-in user on the current client, it means that
    /// another session of the logged-in user left the room with name `roomname`.
    Leave {
        /// The room in question.
        roomname: Box<Word>,
        /// The username of the user that left the room.
        username: Box<Word>,
    },
}

impl PushMessage {
    pub(super) fn parse(s: &str) -> Option<Self> {
        let words: Vec<_> = s.split(' ').collect();
        assert!(words[0] == "_push");
        let item = match words[1] {
            "online" => Self::Online {
                sessions: parsei64(words[2]),
                username: expect_word(words[3]),
            },
            "message" => {
                let message = Message::try_parse(&words[2..]).unwrap();
                Self::Message(message)
            }
            "invite" => Self::Invite {
                roomname: expect_word(words[2]),
                inviter: expect_word(words[3]),
            },
            "join" => Self::Join {
                roomname: expect_word(words[2]),
                username: expect_word(words[3]),
            },
            "leave" => Self::Leave {
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
