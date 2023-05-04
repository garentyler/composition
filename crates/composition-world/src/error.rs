/// This type represents all possible errors that can occur when managing a `World`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the given position was out of bounds")]
    OutOfBounds,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Alias for a Result with the error type `composition_world::Error`.
pub type Result<T> = std::result::Result<T, Error>;
