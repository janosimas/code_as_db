use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Invalid parameter: Parameter should be saved before adding to function")]
    InvalidParameter,
    #[error("Database error")]
    DatabaseError(#[from] butane::Error)
}

pub type Result<T> = std::result::Result<T, Error>;
