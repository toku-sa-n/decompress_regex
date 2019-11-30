fn main() {
    loop {
        let mut line = String::new();
        if std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line")
            == 0
        {
            break;
        }

        line.pop();

        match decompress_regex(&line) {
            Ok(results) => {
                for result in results {
                    println!("{}", result);
                }
            }
            Err(e) => {
                eprintln!("Error decompressing line: {}", e);
            }
        }
    }
}

fn decompress_regex(regex: &str) -> Result<Vec<String>, &'static str> {
    let mut decompressed_strings: Vec<String> = vec![String::from("")];

    check_valid_use_of_question(regex)?;

    for i in 0..regex.chars().count() {
        match regex.chars().nth(i) {
            Some('?') => {
                // `ab??' is a valid regex. It matches with both of 'a' and 'ab'.
                // However if the given sentence contains both of 'a' and 'ab' then
                // it matches 'a', not 'ab'. (not greedy `?')
                if i >= 1 && regex.chars().nth(i - 1) == Some('?') {
                    continue;
                }

                purse_question_mark(&mut decompressed_strings);
            }
            Some('|') => {
                decompressed_strings.append(&mut decompress_regex(&regex[i + 1..])?);
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
    Ok(decompressed_strings)
}

fn check_valid_use_of_question(str: &str) -> Result<(), &'static str> {
    if str.chars().nth(0) == Some('?') {
        return Err("Invalid use of `?'.");
    }

    if str.find("???") != None {
        return Err("Invalid use of `?'.");
    }

    Ok(())
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

    // From https://stackoverflow.com/questions/38183551/concisely-initializing-a-vector-of-strings
    macro_rules! vec_of_strings{
        ($($x:expr),*)=>(vec![$($x.to_string()),*]);
    }

    #[test]
    fn test_string_with_no_regex_symbols() {
        assert_eq!(
            decompress_regex("hello, world!"),
            Ok(vec_of_strings!["hello, world!"])
        );
        assert_eq!(
            decompress_regex("This is a pen."),
            Ok(vec_of_strings!["This is a pen."])
        );
    }

    #[test]
    fn test_string_with_one_question_symbol() {
        assert_eq!(
            decompress_regex("hello!?"),
            Ok(vec_of_strings!["hello!", "hello"])
        );
        assert_eq!(
            decompress_regex("hello!? world"),
            Ok(vec_of_strings!["hello! world", "hello world"]),
        );
        assert_eq!(decompress_regex("a?"), Ok(vec_of_strings!["a", ""]));
    }

    // TODO: Make wrapper of assert_eq! which doesn't consider order of elements.
    #[test]
    fn test_string_with_multiple_question_symbols() {
        assert_eq!(
            decompress_regex("Y?a?hoo!"),
            Ok(vec_of_strings!["Yahoo!", "ahoo!", "Yhoo!", "hoo!"]),
        );
        assert_eq!(
            decompress_regex("a?b?c?"),
            Ok(vec_of_strings!["abc", "bc", "ac", "c", "ab", "b", "a", ""]),
        );
    }

    #[test]
    fn teset_string_with_consecutive_question_symbols() {
        assert_eq!(decompress_regex("ab??"), Ok(vec_of_strings!["ab", "a"]),);
        assert_eq!(
            decompress_regex("a??b?c?"),
            Ok(vec_of_strings!["abc", "bc", "ac", "c", "ab", "b", "a", ""]),
        );
    }

    #[test]
    fn test_string_with_one_bar() {
        assert_eq!(decompress_regex("f|g"), Ok(vec_of_strings!["f", "g"]));
        assert_eq!(decompress_regex("ka|ono"), Ok(vec_of_strings!["ka", "ono"]));
    }

    #[test]
    fn test_string_with_multiple_bar() {
        assert_eq!(
            decompress_regex("a|b|c"),
            Ok(vec_of_strings!["a", "b", "c"])
        );
        assert_eq!(
            decompress_regex("Yahoo!|Google|Bing|Nifty"),
            Ok(vec_of_strings!["Yahoo!", "Google", "Bing", "Nifty"]),
        );
    }

    #[test]
    fn test_with_question_marks_and_bars() {
        assert_eq!(
            decompress_regex("ab?|c"),
            Ok(vec_of_strings!["ab", "a", "c"])
        );
        assert_eq!(
            decompress_regex("abc?|d?ef"),
            Ok(vec_of_strings!["abc", "ab", "def", "ef"]),
        );
    }

    #[test]
    fn test_invalid_use_of_question_mark() {
        assert_eq!(decompress_regex("?this"), Err("Invalid use of `?'."),);
        assert_eq!(decompress_regex("This|?That"), Err("Invalid use of `?'."),);
        assert_eq!(decompress_regex("This???"), Err("Invalid use of `?'."),);
    }
}
