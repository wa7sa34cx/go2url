use crate::validate;
use crate::db::DB;
use dotenv::dotenv;
use std::env;

fn get_line_from_db(filename: &str) -> Result<String, &'static str> {
    if !validate::txt_file(filename) {
        return Err("Invalid file name. Expected: example.txt");
    }

    // dotenv().ok();
    // let db_path = env::var("DB_PATH").unwrap_or_else(|| {
    //     return Err("DB_PATH must be set in .env");
    // });

    // let path = db_path + filename;

    let db = DB::establish(&filename).unwrap_or_else(|e| {
        return Err(e);
    });

    let line = db.get_rand_line().unwrap_or_else(|e| {
        return Err(e);
    });

    line
}