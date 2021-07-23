pub mod error;

use dotenv::dotenv;
use error::DbError;
use rand::seq::SliceRandom;
use std::env;
use tokio::fs;
use tokio::fs::File;

type DbResult<T> = Result<T, DbError>;

// My own simple file read based database
#[derive(Debug)]
pub struct Db {
    file: String,
}

impl Db {
    pub async fn establish(filename: &str) -> DbResult<Db> {
        // Specify in .env where the directory with files is located
        dotenv().ok();
        let db_path = env::var("DB_PATH")?;

        // Create full path to file
        let file = db_path + filename;

        // If file opening doesn't return the Error,
        // so connection can be established
        let _f = File::open(&file).await?;

        Ok(Db { file })
    }

    pub async fn get_rand_line(&self) -> DbResult<String> {
        let contents = fs::read_to_string(&self.file).await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_establish() {
        let db = Db::establish("test.txt").await;
        match db {
            Ok(_) => return,
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn test_get_rand_line() {
        let db = Db::establish("test.txt").await.unwrap();
        let line = db.get_rand_line().await.unwrap();
        match &line[..] {
            "https://google.com" | "https://facebook.com" | "https://amazon.com" => return,
            _ => panic!("Can't get rand line from database"),
        }
    }
}
