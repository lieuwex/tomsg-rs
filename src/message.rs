#[derive(Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub roomname: String,
    pub username: String,
    pub timestamp: i64, // date type?
    pub message: String,
}
