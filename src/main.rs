fn main() {
    println!("Hello, world!");
}

fn decompress_regex(regex: &str) -> Vec<String> {
    vec![regex.to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_with_no_regex_symbols() {
        assert_eq!(vec!["hello, world!"], decompress_regex("hello, world!"));
        assert_eq!(vec!["This is a pen."], decompress_regex("This is a pen."));
    }
}
