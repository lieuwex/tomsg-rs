use super::util::parsei64;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub reply_on: Option<i64>,
    pub roomname: String,
    pub username: String,
    pub timestamp: i64, // date type?
    pub message: String,
}

impl Message {
    pub fn try_parse(words: &[&str]) -> Result<Self, &'static str> {
        // TODO: better error handling

        let roomname = words[0].to_string();
        let username = words[1].to_string();
        let timestamp = parsei64(words[2]);
        let id = parsei64(words[3]);
        let reply_on = match parsei64(words[4]) {
            -1 => None,
            id => Some(id),
        };
        let message = words[5..].join(" ");

        Ok(Self {
            id,
            reply_on,
            roomname,
            username,
            timestamp,
            message,
        })
    }
}
