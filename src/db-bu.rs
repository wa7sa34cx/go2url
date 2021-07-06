// use std::error::Error;
use rand::seq::SliceRandom;
use std::fs;
use std::fs::File;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct DB {
    file: String,
}

impl DB {
    pub fn establish(filename: &str) -> Result<DB, &'static str> {
        dotenv().ok();

        let db_path = match env::var("DB_PATH") {
            Ok(s) => s,
            Err(_) => return Err("DB_PATH must be set in .env"),
        };

        let file = db_path + filename;

        match File::open(&file) {
            Ok(_) => Ok(DB {
                file,
            }),
            Err(_) => Err("Error establishing a database connection"),
        }
    }

    pub fn get_rand_line(&self) -> Result<String, &'static str> {
        let contents = match fs::read_to_string(&self.file) {
            Ok(s) => s,
            Err(_) => return Err("Error reading from database"),
        };

        if contents.is_empty() {
            return Err("Database is empty");
        }

        let lines: Vec<&str> = contents.split('\n').collect();
        let line = lines.choose(&mut rand::thread_rng());

        let line = match line {
            Some(line) => line.to_string(),
            None => return Err("Error with rand"),
        };

        Ok(line)
    }
}