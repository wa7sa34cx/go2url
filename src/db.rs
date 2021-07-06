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
    pub fn establish(file: &str) -> Result<DB, &'static str> {
        match File::open(file) {
            Ok(_) => Ok(DB {
                file: file.to_string(),
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
