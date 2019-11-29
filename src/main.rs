fn main() {
    println!("Hello, world!");
}

fn decompress_regex(regex: &str) -> Vec<String> {
    let mut decompressed_strings: Vec<String> = vec![String::from("")];

    let mut num_kinds = 1;
    for c in regex.chars() {
        if c == '?' {
            for i in 0..num_kinds {
                decompressed_strings.push(decompressed_strings[i].clone());
            }
            num_kinds *= 2;

            for i in num_kinds / 2..num_kinds {
                decompressed_strings[i].pop();
            }
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
        assert_eq!(vec!["a", ""], decompress_regex("a?"));
    }

    // TODO: Make wrapper of assert_eq! which doesn't consider order of elements.
    #[test]
    fn test_string_with_multiple_question_symbols() {
        assert_eq!(
            vec!["Yahoo!", "ahoo!", "Yhoo!", "hoo!"],
            decompress_regex("Y?a?hoo!")
        );
        assert_eq!(
            vec!["abc", "bc", "ac", "c", "ab", "b", "a", ""],
            decompress_regex("a?b?c?")
        );
    }
}
