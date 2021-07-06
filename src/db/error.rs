use std::fmt;

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
            Error::ErrConnection => write!(f, "Error establishing a database connection"),
            Error::ErrReading => write!(f, "Error reading from database"),
            Error::EmptyDB => write!(f, "Database is empty"),
            Error::ErrRand => write!(f, "Error with rand"),
        }
    }
}
