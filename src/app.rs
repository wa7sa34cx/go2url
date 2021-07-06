use crate::db::DB;
use crate::db::error::Error;

pub fn get_rand_line_from_db(filename: &str) -> Result<String, Error> {
    let db = DB::establish(&filename)?;
    let line = db.get_rand_line()?;
    Ok(line)
}