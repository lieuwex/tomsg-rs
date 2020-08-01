pub enum Command {
    Version(String),                   // string
    Register(String, String),          // word, string
    Login(String, String),             // word, string
    Logout,                            //
    Listrooms,                         //
    ListMembers(String),               // word
    CreateRoom,                        //
    Invite(String, String),            // word, word
    Send(String, Option<i64>, String), // word, i64, string
    History(String, i64),              // word, i64
    HistoryBefore(String, i64, i64),   // word, i64, i64
    GetMessage(i64),                   // i64
    Ping,                              //
    IsOnline(String),                  // word
    FirebaseToken(String),             // word
    DeleteFirebaseToken(String),       // word
    UserActive(i64),                   // i64
}

impl Command {
    pub fn to_str(&self) -> String {
        macro_rules! word {
            ($e:expr) => {
                if $e.contains(char::is_whitespace) {
                    panic!("expected word")
                } else {
                    $e
                }
            };
        }

        match self {
            Command::Version(v) => format!("version {}", v),
            Command::Register(username, password) => {
                format!("register {} {}", word!(username), password)
            }
            Command::Login(username, password) => format!("login {} {}", word!(username), password),
            Command::Logout => String::from("logout"),
            Command::Listrooms => String::from("list_rooms"),
            Command::ListMembers(room) => format!("list_members {}", word!(room)),
            Command::CreateRoom => String::from("create_room"),
            Command::Invite(room, user) => format!("invite {} {}", word!(room), word!(user)),
            Command::Send(room, reply_id, message) => {
                let reply_id = reply_id.unwrap_or(-1);
                format!("send {} {} {}", word!(room), reply_id, message)
            }
            Command::History(room, count) => format!("history {} {}", word!(room), count),
            Command::HistoryBefore(room, count, message_id) => {
                format!("history_before {} {} {}", word!(room), count, message_id)
            }
            Command::GetMessage(message_id) => format!("get_message {}", message_id),
            Command::Ping => String::from("ping"),
            Command::IsOnline(user) => format!("is_online {}", word!(user)),
            Command::UserActive(active) => format!("user_active {}", active),

            Command::FirebaseToken(_) => todo!(),
            Command::DeleteFirebaseToken(_) => todo!(),
        }
    }
}
