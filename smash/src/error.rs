#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SendError(Box<dyn std::error::Error>),

    #[error("{0}")]
    RecvError(Box<dyn std::error::Error>),

    #[error("Closed")]
    Closed,

    #[error("Timeout")]
    Timeout,
}

use tokio::sync::mpsc::error::SendError;

impl<T: 'static> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        Self::SendError(Box::new(value))
    }
}

use tokio::sync::oneshot::error::RecvError;

impl From<RecvError> for Error {
    fn from(value: RecvError) -> Self {
        Self::RecvError(Box::new(value))
    }
}