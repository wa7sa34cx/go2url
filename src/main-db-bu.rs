mod db;
use db::DB;
// use dotenv::dotenv;
// use std::env;
// use std::env::VarError;
use std::process;

#[tokio::main]
async fn main() {
    // let db = DB::from("example.txt");

    // let db = DB::from("example.txt").unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // dotenv().ok();
    // let db_path = env::var("DB_PATH").expect("DB_PATH must be set");

    // let path = db_path + "example.txt";
    // let path = db_path + "one-line.txt";
    // let path = db_path + "empty.txt";

    // let db = DB::establish(&path).expect(&format!("Error connecting to {}", path));

    let filename = "example.txt";

    let db = DB::establish(&filename).unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        process::exit(1);
    });
    println!("{:#?}", db);


    let line = db.get_rand_line().unwrap_or_else(|e| {
        eprintln!("{:?}", e);
        process::exit(1);
    });
    println!("{}", line);

}
