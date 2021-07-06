use crate::validate;
use crate::db::DB;
use dotenv::dotenv;
use std::env;

fn get_line_from_db(filename: &str) -> Result<String, &'static str> {
    if !validate::txt_file(filename) {
        return Err("Invalid file name. Expected: example.txt");
    }

    let db = DB::establish(&filename).unwrap_or_else(|e| {
        return Err(e);
    });

    let line = db.get_rand_line().unwrap_or_else(|e| {
        return Err(e);
    });

    line
}