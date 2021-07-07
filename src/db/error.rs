use std::fmt;
use std::error;
use std::io;
use std::env;

#[derive(Debug)]
pub enum DbError {
    VarError(env::VarError),
    IoError(io::Error),
    EmptyDb,
    ErrRand,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DbError::VarError(ref error) => write!(f, "{}", error),
            DbError::IoError(ref error) => write!(f, "{}", error),
            DbError::EmptyDb => write!(f, "database is empty"),
            DbError::ErrRand => write!(f, "error with rand"),
        }
    }
}

impl error::Error for DbError {}

impl From<env::VarError> for DbError {
    fn from(error: env::VarError) -> Self {
        DbError::VarError(error)
    }
}

impl From<io::Error> for DbError {
    fn from(error: io::Error) -> Self {
        DbError::IoError(error)
    }
}