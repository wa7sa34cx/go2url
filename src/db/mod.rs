pub mod error;

use dotenv::dotenv;
use error::Error;
use rand::seq::SliceRandom;
use std::env;
use std::fs;
use std::fs::File;

type DBResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct DB {
    file: String,
}

impl DB {
    pub fn establish(filename: &str) -> DBResult<DB> {
        dotenv().ok();

        let db_path = match env::var("DB_PATH") {
            Ok(s) => s,
            Err(_) => return Err(Error::NoEnv),
        };

        let file = db_path + filename;

        match File::open(&file) {
            Ok(_) => Ok(DB { file }),
            Err(_) => Err(Error::ErrConnection),
        }
    }

    pub fn get_rand_line(&self) -> DBResult<String> {
        let contents = match fs::read_to_string(&self.file) {
            Ok(s) => s,
            Err(_) => return Err(Error::ErrReading),
        };

        if contents.is_empty() {
            return Err(Error::EmptyDB);
        }

        let lines: Vec<&str> = contents.split('\n').collect();
        let line = lines.choose(&mut rand::thread_rng());

        match line {
            Some(line) => Ok(line.to_string()),
            None => Err(Error::ErrRand),
        }
    }
}
