use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    EmptyQuery,
    NotValidQuery,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::EmptyQuery => write!(f, "expected filename in query"),
            AppError::NotValidQuery => write!(f, "invalid filename, expected: example.txt"),
        }
    }
}

impl error::Error for AppError {}
