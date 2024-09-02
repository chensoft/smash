#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Unknown(String),

    #[error("Closed")]
    Closed,

    #[error("Timeout")]
    Timeout,
}