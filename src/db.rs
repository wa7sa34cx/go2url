use std::error::Error;
use std::fs;
// use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct DB {
    path: String,
}

impl DB {
    pub fn establish(path: &str) -> Result<DB, Box<dyn Error>> {
        let _contents = fs::read_to_string(path)?;

        Ok(DB {
            path: path.to_string(),
        })
    }
}
