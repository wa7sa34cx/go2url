mod temp;

use std::env;
use std::env::VarError;

fn main() {
    // let error = temp().unwrap_or_else(|e| {
    //     print_error(&e.to_string());
    // });

    // let error = temp();
    // println!("{:?}", error);

    // let error = match temp() {
    //     Ok(_) => println!("Ok"),
    //     Err(e) => print_error(&e.to_string()),
    // };

    temp::foo::main();
}

fn temp() -> Result<String, VarError> {
    let s = env::var("TEMP")?;
    Ok(s)
}

fn print_error(error: &str) {
    println!("{}", error);
}