use std::borrow::Cow;
use std::time;

use crate::id::Id;
use crate::line::Line;
use crate::word::Word;

/// A command that is sendable to a tomsg server, with related information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command<'a> {
    Version(Cow<'a, Word>),
    Register {
        username: Cow<'a, Word>,
        password: Cow<'a, Line>,
    },
    Login {
        username: Cow<'a, Word>,
        password: Cow<'a, Line>,
    },
    ChangePassword(Cow<'a, Line>),
    Logout,
    ListRooms,
    ListMembers {
        roomname: Cow<'a, Word>,
    },
    CreateRoom,
    LeaveRoom(Cow<'a, Word>),
    Invite {
        roomname: Cow<'a, Word>,
        username: Cow<'a, Word>,
    },
    Send {
        roomname: Cow<'a, Word>,
        reply_on: Option<Id>,
        message: Cow<'a, Line>,
    },
    SendAt {
        apikey: Cow<'a, Word>,
        roomname: Cow<'a, Word>,
        reply_on: Option<Id>,
        timestamp: time::SystemTime,
        message: Cow<'a, Line>,
    },
    History {
        roomname: Cow<'a, Word>,
        count: i64,
    },
    HistoryBefore {
        roomname: Cow<'a, Word>,
        count: i64,
        message_id: Id,
    },
    GetMessage(Id),
    Ping,
    IsOnline {
        username: Cow<'a, Word>,
    },
    FirebaseToken(Cow<'a, Word>),
    DeleteFirebaseToken(Cow<'a, Word>),
    UserActive(i64),
}

impl<'a> Command<'a> {
    #[allow(clippy::inherent_to_string)]
    pub(super) fn to_string(&self) -> String {
        match self {
            Command::Version(v) => format!("version {}", v),
            Command::Register { username, password } => {
                format!("register {} {}", username, password)
            }
            Command::Login { username, password } => format!("login {} {}", username, password),
            Command::ChangePassword(password) => format!("change_password {}", password),
            Command::Logout => String::from("logout"),
            Command::ListRooms => String::from("list_rooms"),
            Command::ListMembers { roomname } => format!("list_members {}", roomname),
            Command::CreateRoom => String::from("create_room"),
            Command::LeaveRoom(roomname) => format!("leave_room {}", roomname),
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
            Command::SendAt {
                apikey,
                roomname,
                reply_on,
                timestamp,
                message,
            } => {
                let reply_on: i64 = match reply_on {
                    None => -1,
                    Some(id) => id.into(),
                };
                format!(
                    "sendat {} {} {} {} {}",
                    apikey,
                    roomname,
                    reply_on,
                    timestamp
                        .duration_since(time::UNIX_EPOCH)
                        .unwrap()
                        .as_micros(),
                    message
                )
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
