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

    #[test]
    fn test_string_with_one_bar() {
        assert_eq!(vec!["f", "g"], decompress_regex("f|g"));
        assert_eq!(vec!["ka", "ono"], decompress_regex("ka|ono"));
    }
}
