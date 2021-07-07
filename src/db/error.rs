use std::fmt;
use std::error::Error;
use std::env::VarError;

#[derive(Debug)]
pub enum DbError {
    NoEnv,
    ErrConnection,
    ErrReading,
    EmptyDb,
    ErrRand,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DbError::NoEnv => write!(f, "DB_PATH must be set in .env"),
            DbError::ErrConnection => write!(f, "database file not found"),
            DbError::ErrReading => write!(f, "error reading from database"),
            DbError::EmptyDb => write!(f, "database is empty"),
            DbError::ErrRand => write!(f, "error with rand"),
        }
    }
}

impl Error for DbError {}

impl From<VarError> for DbError {
    fn from(_: VarError) -> Self {
        DbError::NoEnv
    }
}