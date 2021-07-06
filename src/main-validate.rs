mod validate;

#[tokio::main]
async fn main() {
    // let query = String::from("example.txt");
    let s = String::from("1239-hh_abc.txt");
    println!("{}", validate::txt_file(&s));
}
