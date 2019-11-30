fn main() {
    println!("Hello, world!");
}

fn decompress_regex(regex: &str) -> Result<Vec<String>, &'static str> {
    let mut decompressed_strings: Vec<String> = vec![String::from("")];

    for i in 0..regex.chars().count() {
        match regex.chars().nth(i) {
            Some('?') => purse_question_mark(&mut decompressed_strings),
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
        assert_eq!(
            decompress_regex("hello, world!"),
            Ok(vec![String::from("hello, world!")])
        );
        assert_eq!(
            decompress_regex("This is a pen."),
            Ok(vec![String::from("This is a pen.")])
        );
    }

    #[test]
    fn test_string_with_one_question_symbol() {
        assert_eq!(
            decompress_regex("hello!?"),
            Ok(vec![String::from("hello!"), String::from("hello")])
        );
        assert_eq!(
            decompress_regex("hello!? world"),
            Ok(vec![
                String::from("hello! world"),
                String::from("hello world")
            ]),
        );
        assert_eq!(
            decompress_regex("a?"),
            Ok(vec![String::from("a"), String::from("")])
        );
    }

    // TODO: Make wrapper of assert_eq! which doesn't consider order of elements.
    #[test]
    fn test_string_with_multiple_question_symbols() {
        assert_eq!(
            decompress_regex("Y?a?hoo!"),
            Ok(vec![
                String::from("Yahoo!"),
                String::from("ahoo!"),
                String::from("Yhoo!"),
                String::from("hoo!")
            ]),
        );
        assert_eq!(
            decompress_regex("a?b?c?"),
            Ok(vec![
                String::from("abc"),
                String::from("bc"),
                String::from("ac"),
                String::from("c"),
                String::from("ab"),
                String::from("b"),
                String::from("a"),
                String::from("")
            ]),
        );
    }

    #[test]
    fn test_string_with_one_bar() {
        assert_eq!(
            decompress_regex("f|g"),
            Ok(vec![String::from("f"), String::from("g")])
        );
        assert_eq!(
            decompress_regex("ka|ono"),
            Ok(vec![String::from("ka"), String::from("ono")])
        );
    }

    #[test]
    fn test_string_with_multiple_bar() {
        assert_eq!(
            decompress_regex("a|b|c"),
            Ok(vec![
                String::from("a"),
                String::from("b"),
                String::from("c")
            ])
        );
        assert_eq!(
            decompress_regex("Yahoo!|Google|Bing|Nifty"),
            Ok(vec![
                String::from("Yahoo!"),
                String::from("Google"),
                String::from("Bing"),
                String::from("Nifty")
            ]),
        );
    }

    #[test]
    fn test_with_question_marks_and_bars() {
        assert_eq!(
            decompress_regex("ab?|c"),
            Ok(vec![
                String::from("ab"),
                String::from("a"),
                String::from("c")
            ])
        );
        assert_eq!(
            decompress_regex("abc?|d?ef"),
            Ok(vec![
                String::from("abc"),
                String::from("ab"),
                String::from("def"),
                String::from("ef")
            ]),
        );
    }
}
