use regex::Regex;

pub fn txt_file(s: &str) -> bool {
    let re = Regex::new(r"^[\w\-. ]+\.txt$").unwrap();
    re.is_match(s)
}
