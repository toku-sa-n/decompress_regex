fn main() {
    println!("Hello, world!");
}

fn decompress_regex(regex: &str) -> Vec<String> {
    let mut decompressed_strings: Vec<String> = vec![String::from("")];

    let current_idx = 0;
    let mut num_kinds = 1;
    for c in regex.chars() {
        if c == '?' {
            decompressed_strings.push(decompressed_strings[current_idx].clone());
            num_kinds += 1;

            decompressed_strings[current_idx + 1].pop();
        } else {
            for idx in 0..num_kinds {
                decompressed_strings[idx].push(c);
            }
        }
    }
    decompressed_strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_with_no_regex_symbols() {
        assert_eq!(vec!["hello, world!"], decompress_regex("hello, world!"));
        assert_eq!(vec!["This is a pen."], decompress_regex("This is a pen."));
    }

    #[test]
    fn test_string_with_one_question_symbol() {
        assert_eq!(vec!["hello!", "hello"], decompress_regex("hello!?"));
        assert_eq!(
            vec!["hello! world", "hello world"],
            decompress_regex("hello!? world")
        );
    }
}
