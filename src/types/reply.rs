use super::message::Message;

#[derive(Debug)]
pub enum ReplyCommand {
    Ok,                    //
    Number(i64),           // i64
    Error(String),         // string
    Name(String),          // word
    List(Vec<String>),     // words
    Pong,                  //
    History(Vec<Message>), //

    historyInternal(i64),
    historyMessageInternal(i64, Message), // index, message
}

#[derive(Debug)]
pub struct Reply {
    pub tag: String,
    pub command: ReplyCommand,
}

impl Reply {
    pub fn parse(s: &str) -> Self {
        let words: Vec<_> = s.split(' ').collect();

        let make = |command: ReplyCommand| -> Reply {
            Self {
                tag: words[0].to_owned(),
                command: command,
            }
        };

        let parsei64 = |item: &str| -> i64 { item.parse::<i64>().unwrap() };

        match words[1] {
            "ok" => make(ReplyCommand::Ok),
            "number" => make(ReplyCommand::Number(parsei64(words[2]))),
            "error" => make(ReplyCommand::Error(words[2..].join(" "))),
            "name" => make(ReplyCommand::Name(words[2].to_string())),
            "list" => make(ReplyCommand::List(
                words[2..].iter().map(|w| w.to_string()).collect(),
            )),
            "pong" => make(ReplyCommand::Pong),

            // still needs to be handled
            "history" => make(ReplyCommand::historyInternal(parsei64(words[2]))),
            "history_message" => {
                let index = parsei64(words[2]);
                let roomname = words[3].to_string();
                let username = words[4].to_string();
                let timestamp = parsei64(words[5]);
                let id = parsei64(words[6]);
                let message = words[7..].join(" ");

                make(ReplyCommand::historyMessageInternal(
                    index,
                    Message {
                        id,
                        roomname,
                        username,
                        timestamp,
                        message,
                    },
                ))
            }

            w => panic!(format!("unexpected response type: '{}'", w)),
        }
    }
}
