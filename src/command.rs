use crate::id::Id;
use crate::line::Line;
use crate::word::Word;

/// A command that is sendable to a tomsg server, with related information.
pub enum Command {
    Version(Word),
    Register {
        username: Word,
        password: Line,
    },
    Login {
        username: Word,
        password: Line,
    },
    Logout,
    ListRooms,
    ListMembers {
        roomname: Word,
    },
    CreateRoom,
    Invite {
        roomname: Word,
        username: Word,
    },
    Send {
        roomname: Word,
        reply_on: Option<Id>,
        message: Line,
    },
    History {
        roomname: Word,
        count: i64,
    },
    HistoryBefore {
        roomname: Word,
        count: i64,
        message_id: Id,
    },
    GetMessage(Id),
    Ping,
    IsOnline {
        username: Word,
    },
    FirebaseToken(Word),
    DeleteFirebaseToken(Word),
    UserActive(i64),
}

impl Command {
    #[allow(clippy::inherent_to_string)]
    pub(super) fn to_string(&self) -> String {
        match self {
            Command::Version(v) => format!("version {}", v),
            Command::Register { username, password } => {
                format!("register {} {}", username, password)
            }
            Command::Login { username, password } => format!("login {} {}", username, password),
            Command::Logout => String::from("logout"),
            Command::ListRooms => String::from("list_rooms"),
            Command::ListMembers { roomname } => format!("list_members {}", roomname),
            Command::CreateRoom => String::from("create_room"),
            Command::Invite { roomname, username } => format!("invite {} {}", roomname, username),
            Command::Send {
                roomname,
                reply_on,
                message,
            } => {
                let reply_on: i64 = match reply_on {
                    None => -1,
                    Some(id) => id.into(),
                };
                format!("send {} {} {}", roomname, reply_on, message)
            }
            Command::History { roomname, count } => format!("history {} {}", roomname, count),
            Command::HistoryBefore {
                roomname,
                count,
                message_id,
            } => format!("history_before {} {} {}", roomname, count, message_id),
            Command::GetMessage(message_id) => format!("get_message {}", message_id),
            Command::Ping => String::from("ping"),
            Command::IsOnline { username } => format!("is_online {}", username),
            Command::FirebaseToken(token) => format!("firebase_token {}", token),
            Command::DeleteFirebaseToken(token) => format!("delete_firebase_token {}", token),
            Command::UserActive(active) => format!("user_active {}", active),
        }
    }
}
