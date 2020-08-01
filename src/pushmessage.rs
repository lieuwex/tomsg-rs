use crate::message::Message;

#[derive(Debug)]
pub enum PushMessage {
    Online(i64, String), // i64, word
    Message(Message),
    Invite(String, String), // word, word
    Join(String, String),   // word, word
}

impl PushMessage {
    pub fn parse(s: &str) -> Option<Self> {
        let parsei64 = |item: &str| -> i64 { item.parse::<i64>().unwrap() };

        let words: Vec<_> = s.split(' ').collect();
        assert!(words[0] == "_push");
        let item = match words[1] {
            "online" => PushMessage::Online(parsei64(&words[2]), words[3].to_owned()),
            "message" => {
                let message = Message::try_parse(&words[2..]).unwrap();
                PushMessage::Message(message)
            }
            "invite" => PushMessage::Invite(words[2].to_owned(), words[3].to_owned()),
            "join" => PushMessage::Join(words[2].to_owned(), words[3].to_owned()),

            // we can ignore this
            "ping" => return None,

            _ => panic!("unknown push type"),
        };

        Some(item)
    }
}
