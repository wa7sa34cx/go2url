use std::env;
use dotenv::dotenv;

pub fn go() {
    dotenv().ok();

    let path = env::var("PATH_TO_GO_FOLDER");
    println!("{:?}", path);
}