pub enum Command<'a> {
    Version(&'a str),                 // string
    Register(&'a str, &'a str),       // word, string
    Login(&'a str, &'a str),          // word, string
    Logout,                           //
    Listrooms,                        //
    ListMembers(&'a str),             // word
    CreateRoom,                       //
    Invite(&'a str, &'a str),         // word, word
    Send(&'a str, &'a str),           // word, string
    History(&'a str, i64),            // word, i64
    HistoryBefore(&'a str, i64, i64), // word, i64, i64
    Ping,                             //
    IsOnline(&'a str),                // word
    FirebaseToken(&'a str),           // word
    DeleteFirebaseToken(&'a str),     // word
    UserActive(i64),                  // i64
}

impl<'a> Command<'a> {
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
            Command::Send(room, message) => format!("send {} {}", word!(room), message),
            Command::History(room, count) => format!("history {} {}", word!(room), count),
            Command::HistoryBefore(room, count, message_id) => {
                format!("history_before {} {} {}", word!(room), count, message_id)
            }
            Command::Ping => String::from("ping"),
            Command::IsOnline(user) => format!("is_oline {}", word!(user)),
            Command::UserActive(active) => format!("user_active {}", active),

            Command::FirebaseToken(_) => todo!(),
            Command::DeleteFirebaseToken(_) => todo!(),
        }
    }
}
