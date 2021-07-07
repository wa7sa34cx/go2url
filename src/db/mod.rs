pub mod error;

use dotenv::dotenv;
use error::DbError;
use rand::seq::SliceRandom;
use std::env;
use tokio::fs::File;
use tokio::fs;

type DbResult<T> = Result<T, DbError>;

#[derive(Debug)]
pub struct Db {
    file: String,
}

impl Db {
    pub async fn establish(filename: &str) -> DbResult<Db> {
        dotenv().ok();

        let db_path = env::var("DB_PATH")?;

        let file = db_path + filename;

        match File::open(&file).await {
            Ok(_) => Ok(Db { file }),
            Err(_) => Err(DbError::ErrConnection),
        }
    }

    pub async fn get_rand_line(&self) -> DbResult<String> {
        let contents = match fs::read_to_string(&self.file).await {
            Ok(s) => s,
            Err(_) => return Err(DbError::ErrReading),
        };

        if contents.is_empty() {
            return Err(DbError::EmptyDb);
        }

        let lines: Vec<&str> = contents.split('\n').collect();
        let line = lines.choose(&mut rand::thread_rng());

        match line {
            Some(line) => Ok(line.to_string()),
            None => Err(DbError::ErrRand),
        }
    }
}
