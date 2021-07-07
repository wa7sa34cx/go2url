use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    NoEnv,
    ErrConnection,
    ErrReading,
    EmptyDB,
    ErrRand,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::NoEnv => write!(f, "DB_PATH must be set in .env"),
            Error::ErrConnection => write!(f, "database file not found"),
            Error::ErrReading => write!(f, "error reading from database"),
            Error::EmptyDB => write!(f, "database is empty"),
            Error::ErrRand => write!(f, "error with rand"),
        }
    }
}

impl error::Error for Error {}