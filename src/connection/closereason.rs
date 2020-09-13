/// The reason a `Connection` was closed.
#[derive(Clone, Debug)]
pub enum CloseReason {
    /// The connection socket hit an EOF.
    EOF,
    /// An unknown error occured.
    Err(String),
}
