#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SendError(String),

    #[error("{0}")]
    RecvError(String),

    #[error("Closed")]
    Closed,

    #[error("Timeout")]
    Timeout,
}

use tokio::sync::mpsc::error::{TrySendError, SendError};

impl<T> From<TrySendError<T>> for Error {
    fn from(value: TrySendError<T>) -> Self {
        Self::SendError(value.to_string())
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        Self::SendError(value.to_string())
    }
}

use tokio::sync::oneshot::error::RecvError;

impl From<RecvError> for Error {
    fn from(value: RecvError) -> Self {
        Self::RecvError(value.to_string())
    }
}