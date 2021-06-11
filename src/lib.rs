use dotenv::dotenv;
use std::env;

pub fn go() {
    dotenv().ok();

    let path = env::var("PATH_TO_GO_FOLDER");
    println!("{:?}", path);
}
