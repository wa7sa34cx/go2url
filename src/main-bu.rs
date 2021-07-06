mod db;

use db::DB;
use std::process;

fn main() {
    // let filename = String::from("example2.txt");
    let filename = String::from("empty.txt");

    let db = DB::establish(&filename).unwrap_or_else(|e| {
        // eprintln!("{}", e);
        print_error(&e.to_string());
        process::exit(1);
    });
    println!("{:#?}", db);

    let line = db.get_rand_line().unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        process::exit(1);
    });
    println!("{}", line);
}

fn print_error(error: &str) {
    println!("{}", error);
}
