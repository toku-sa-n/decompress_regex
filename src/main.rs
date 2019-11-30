fn main() {
    println!("Hello, world!");
}

fn decompress_regex(regex: &str) -> Vec<String> {
    let mut decompressed_strings: Vec<String> = vec![String::from("")];

    for i in 0..regex.chars().count() {
        match regex.chars().nth(i) {
            Some('?') => purse_question_mark(&mut decompressed_strings),
            Some('|') => {
                decompressed_strings.append(&mut decompress_regex(&regex[i + 1..]));
                break;
            }
            Some(c) => {
                for idx in 0..decompressed_strings.len() {
                    decompressed_strings[idx].push(c);
                }
            }
            _ => (),
        }
    }
    decompressed_strings
}

// TODO: Deal with side effects.
fn purse_question_mark(strings: &mut Vec<String>) -> () {
    for i in 0..strings.len() {
        strings.push(strings[i].clone());
    }

    for i in strings.len() / 2..strings.len() {
        strings[i].pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_with_no_regex_symbols() {
        assert_eq!(decompress_regex("hello, world!"), vec!["hello, world!"]);
        assert_eq!(decompress_regex("This is a pen."), vec!["This is a pen."]);
    }

    #[test]
    fn test_string_with_one_question_symbol() {
        assert_eq!(decompress_regex("hello!?"), vec!["hello!", "hello"]);
        assert_eq!(
            decompress_regex("hello!? world")
            vec!["hello! world", "hello world"],
        );
        assert_eq!(decompress_regex("a?"), vec!["a", ""]);
    }

    // TODO: Make wrapper of assert_eq! which doesn't consider order of elements.
    #[test]
    fn test_string_with_multiple_question_symbols() {
        assert_eq!(
            decompress_regex("Y?a?hoo!")
            vec!["Yahoo!", "ahoo!", "Yhoo!", "hoo!"],
        );
        assert_eq!(
            decompress_regex("a?b?c?")
            vec!["abc", "bc", "ac", "c", "ab", "b", "a", ""],
        );
    }

    #[test]
    fn test_string_with_one_bar() {
        assert_eq!(decompress_regex("f|g"), vec!["f", "g"]);
        assert_eq!(decompress_regex("ka|ono"), vec!["ka", "ono"]);
    }

    #[test]
    fn test_string_with_multiple_bar() {
        assert_eq!(decompress_regex("a|b|c"), vec!["a", "b", "c"]);
        assert_eq!(
            decompress_regex("Yahoo!|Google|Bing|Nifty")
            vec!["Yahoo!", "Google", "Bing", "Nifty"],
        );
    }

    #[test]
    fn test_with_question_marks_and_bars() {
        assert_eq!(decompress_regex("ab?|c"), vec!["ab", "a", "c"]);
        assert_eq!(
            decompress_regex("abc?|d?ef")
            vec!["abc", "ab", "def", "ef"],
        );
    }
}
