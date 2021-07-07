use crate::db::DB;
use crate::db::error::Error;
// use std::error::Error;
use tokio::fs::File;
use tokio::io;

pub async fn get_rand_line_from_db(filename: &str) -> Result<String, Error> {
    let db = DB::establish(&filename)?;
    let line = db.get_rand_line()?;
    Ok(line)
}