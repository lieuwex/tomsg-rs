use crate::id::Id;
use crate::line::Line;
use crate::word::Word;

/// A command that is sendable to a tomsg server, with related information.
pub enum Command {
    Version(Word),                // word
    Register(Word, Line),         // word, string
    Login(Word, Line),            // word, string
    Logout,                       //
    Listrooms,                    //
    ListMembers(Word),            // word
    CreateRoom,                   //
    Invite(Word, Word),           // word, word
    Send(Word, Option<Id>, Line), // word, i64, string
    History(Word, i64),           // word, i64
    HistoryBefore(Word, i64, Id), // word, i64, i64
    GetMessage(i64),              // i64
    Ping,                         //
    IsOnline(Word),               // word
    FirebaseToken(Word),          // word
    DeleteFirebaseToken(Word),    // word
    UserActive(i64),              // i64
}

impl Command {
    pub(super) fn to_string(&self) -> String {
        match self {
            Command::Version(v) => format!("version {}", v),
            Command::Register(username, password) => format!("register {} {}", username, password),
            Command::Login(username, password) => format!("login {} {}", username, password),
            Command::Logout => String::from("logout"),
            Command::Listrooms => String::from("list_rooms"),
            Command::ListMembers(room) => format!("list_members {}", room),
            Command::CreateRoom => String::from("create_room"),
            Command::Invite(room, user) => format!("invite {} {}", room, user),
            Command::Send(room, reply_id, message) => {
                let reply_id: i64 = match reply_id {
                    None => -1,
                    Some(id) => id.into(),
                };
                format!("send {} {} {}", room, reply_id, message)
            }
            Command::History(room, count) => format!("history {} {}", room, count),
            Command::HistoryBefore(room, count, message_id) => {
                format!("history_before {} {} {}", room, count, message_id)
            }
            Command::GetMessage(message_id) => format!("get_message {}", message_id),
            Command::Ping => String::from("ping"),
            Command::IsOnline(user) => format!("is_online {}", user),
            Command::UserActive(active) => format!("user_active {}", active),

            Command::FirebaseToken(_) => todo!(),
            Command::DeleteFirebaseToken(_) => todo!(),
        }
    }
}
