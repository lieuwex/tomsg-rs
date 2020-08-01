#[derive(Clone, Debug)]
pub enum CloseReason {
    EOF,
    Err(String),
}
