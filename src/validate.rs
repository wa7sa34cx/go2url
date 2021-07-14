use regex::Regex;

pub fn txt_file(s: &str) -> bool {
    let re = Regex::new(r"^[\w\-. ]+\.txt$").unwrap();
    re.is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txt_file() {
        // valid txt files
        assert!(txt_file("example.txt"));
        assert!(txt_file("123-abc_456.txt"));

        // not valid txt files
        assert!(!txt_file("/etc/hosts"));
        assert!(!txt_file("example.exe"));
        assert!(!txt_file("../../file.txt"));
        assert!(!txt_file("hello world"));
    }
}
