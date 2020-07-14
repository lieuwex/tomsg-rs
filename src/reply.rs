use super::message::Message;

pub enum InternalReplyCommand {
    Normal(ReplyCommand),
    HistoryInit(i64),
    HistoryMessage(i64, Message), // index, message
}

#[derive(Debug)]
pub enum ReplyCommand {
    Ok,                    //
    Number(i64),           // i64
    Error(String),         // string
    Name(String),          // word
    List(Vec<String>),     // words
    Pong,                  //
    History(Vec<Message>), //
}

#[derive(Debug)]
pub struct Reply {
    pub tag: String,
    pub command: ReplyCommand,
}

pub fn parse(s: &str) -> (String, InternalReplyCommand) {
    let words: Vec<_> = s.split(' ').collect();

    let make = |command: InternalReplyCommand| -> (String, InternalReplyCommand) {
        (words[0].to_string(), command)
    };
    let make_normal = |command: ReplyCommand| -> (String, InternalReplyCommand) {
        make(InternalReplyCommand::Normal(command))
    };

    let parsei64 = |item: &str| -> i64 { item.parse::<i64>().unwrap() };

    match words[1] {
        "ok" => make_normal(ReplyCommand::Ok),
        "number" => make_normal(ReplyCommand::Number(parsei64(words[2]))),
        "error" => make_normal(ReplyCommand::Error(words[2..].join(" "))),
        "name" => make_normal(ReplyCommand::Name(words[2].to_string())),
        "list" => make_normal(ReplyCommand::List(
            words[3..].iter().map(|w| w.to_string()).collect(),
        )),
        "pong" => make_normal(ReplyCommand::Pong),

        // still needs to be handled
        "history" => make(InternalReplyCommand::HistoryInit(parsei64(words[2]))),
        "history_message" => {
            let index = parsei64(words[2]);
            let roomname = words[3].to_string();
            let username = words[4].to_string();
            let timestamp = parsei64(words[5]);
            let id = parsei64(words[6]);
            let message = words[7..].join(" ");

            make(InternalReplyCommand::HistoryMessage(
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
