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
        room_name: Word,
    },
    CreateRoom,
    Invite {
        room_name: Word,
        user: Word,
    },
    Send {
        room_name: Word,
        reply_on: Option<Id>,
        message: Line,
    },
    History {
        room_name: Word,
        count: i64,
    },
    HistoryBefore {
        room_name: Word,
        count: i64,
        message_id: Id,
    },
    GetMessage(Id),
    Ping,
    IsOnline {
        user: Word,
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
            Command::ListMembers { room_name } => format!("list_members {}", room_name),
            Command::CreateRoom => String::from("create_room"),
            Command::Invite { room_name, user } => format!("invite {} {}", room_name, user),
            Command::Send {
                room_name,
                reply_on,
                message,
            } => {
                let reply_on: i64 = match reply_on {
                    None => -1,
                    Some(id) => id.into(),
                };
                format!("send {} {} {}", room_name, reply_on, message)
            }
            Command::History { room_name, count } => format!("history {} {}", room_name, count),
            Command::HistoryBefore {
                room_name,
                count,
                message_id,
            } => format!("history_before {} {} {}", room_name, count, message_id),
            Command::GetMessage(message_id) => format!("get_message {}", message_id),
            Command::Ping => String::from("ping"),
            Command::IsOnline { user } => format!("is_online {}", user),
            Command::FirebaseToken(token) => format!("firebase_token {}", token),
            Command::DeleteFirebaseToken(token) => format!("delete_firebase_token {}", token),
            Command::UserActive(active) => format!("user_active {}", active),
        }
    }
}
