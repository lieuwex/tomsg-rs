use crate::id::Id;
use crate::line::Line;
use crate::word::Word;

/// A command that is sendable to a tomsg server, with related information.
pub enum Command<'a> {
    Version(&'a Word),                    // word
    Register(&'a Word, &'a Line),         // word, string
    Login(&'a Word, &'a Line),            // word, string
    Logout,                               //
    Listrooms,                            //
    ListMembers(&'a Word),                // word
    CreateRoom,                           //
    Invite(&'a Word, &'a Word),           // word, word
    Send(&'a Word, Option<Id>, &'a Line), // word, i64, string
    History(&'a Word, i64),               // word, i64
    HistoryBefore(&'a Word, i64, Id),     // word, i64, i64
    GetMessage(i64),                      // i64
    Ping,                                 //
    IsOnline(&'a Word),                   // word
    FirebaseToken(&'a Word),              // word
    DeleteFirebaseToken(&'a Word),        // word
    UserActive(i64),                      // i64
}

impl<'a> Command<'a> {
    pub(super) fn to_string(&'a self) -> String {
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
