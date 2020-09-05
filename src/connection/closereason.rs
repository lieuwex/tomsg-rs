#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CloseReason {
    EOF,
    Err(String),
}
