use std::{fmt::{Formatter, Display}, error::Error};

/// a list of custom error variants
#[derive(Debug)]
pub enum CustomError {
    /// database-related errors (from MongoDB)
    DatabaseError(String),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CustomError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl Error for CustomError {}

impl From<mongodb::error::Error> for CustomError {
    fn from(err: mongodb::error::Error) -> Self {
        CustomError::DatabaseError(err.to_string())
    }
}