use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct ParsedText {
    pub highlight: String,
    pub color: Option<String>,
    pub message: Option<String>,
    pub separator: Option<String>,
}

const WS: char = ' ';
const LF: char = '\n';
const WHITE: &str = "white";
const YELLOW: &str = "yellow";

fn separate(text: String) -> VecDeque<String> {
    let mut separated: VecDeque<String> = VecDeque::from(vec![String::new()]);

    for (_, c) in text.char_indices().into_iter() {
        let elements = separated.pop_back().unwrap();
        let last_element = elements.chars().last().unwrap_or_default();

        let is_same = match c {
            WS | LF => match last_element {
                WS | LF => true,
                _ => false,
            },
            _ => match last_element {
                WS | LF => false,
                _ => true,
            },
        };

        if is_same {
            separated.push_back(format!("{}{}", elements, c));
        } else {
            separated.push_back(elements);
            separated.push_back(c.to_string());
        }
    }

    separated
}

pub fn text_parse(text: String) -> ParsedText {
    let mut separated = separate(text);

    let highlight = separated.pop_front().expect("Highlight text is necessary");
    let sep1 = separated.pop_front();
    let color_or_word = separated.pop_front();
    let sep2 = separated.pop_front();
    let message = separated.into_iter().collect::<String>();

    let sep1 = if let Some(sep) = sep1 {
        if sep.contains('\n') {
            return ParsedText {
                highlight,
                color: None,
                message: Some(format!(
                    "{}{}{}",
                    color_or_word.unwrap_or_default(),
                    sep2.unwrap_or_default(),
                    message
                )),
                separator: Some(sep),
            };
        } else {
            Some(sep)
        }
    } else {
        return ParsedText {
            highlight,
            color: None,
            message: None,
            separator: None,
        };
    };

    let (color, word) = if let Some(color_or_word) = color_or_word {
        match color_or_word.as_str() {
            WHITE | YELLOW => (Some(color_or_word), None),
            _ => (None, Some(color_or_word)),
        }
    } else {
        return ParsedText {
            highlight,
            color: None,
            message: None,
            separator: None,
        };
    };

    if let Some(word) = word {
        return ParsedText {
            highlight,
            color: None,
            message: Some(format!("{}{}{}", word, sep2.unwrap_or_default(), message)),
            separator: sep1,
        };
    }

    if message.is_empty() {
        return ParsedText {
            highlight,
            color,
            message: None,
            separator: None,
        };
    }

    ParsedText {
        highlight,
        color,
        message: Some(message),
        separator: sep2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod test_separate {
        use super::*;

        #[test]
        fn test_separate_space() {
            let actual = separate(String::from("a b c"));
            assert_eq!(actual, vec!["a", " ", "b", " ", "c"]);
        }

        #[test]
        fn test_separate_linefeed() {
            let actual = separate(String::from("a\nb\nc"));
            assert_eq!(actual, vec!["a", "\n", "b", "\n", "c"]);
        }

        #[test]
        fn test_separate_space_and_linefeed() {
            let actual = separate(String::from("a b\nc \nd\n ef \n gh\n \nij"));
            assert_eq!(
                actual,
                vec![
                    "a", " ", "b", "\n", "c", " \n", "d", "\n ", "ef", " \n ", "gh", "\n \n", "ij"
                ]
            );
        }
    }

    #[cfg(test)]
    mod test_parse_highlight {
        use super::*;

        #[test]
        fn test_parse_highlight_only() {
            let actual = text_parse(String::from("abc"));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("abc"),
                    color: None,
                    message: None,
                    separator: None,
                }
            );
        }

        #[test]
        fn test_parse_highlight_and_white() {
            let actual = text_parse(String::from("abc white"));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("abc"),
                    color: Some(String::from("white")),
                    message: None,
                    separator: None,
                }
            );
        }

        #[test]
        fn test_parse_highlight_and_yellow() {
            let actual = text_parse(String::from("xyz yellow"));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("xyz"),
                    color: Some(String::from("yellow")),
                    message: None,
                    separator: None,
                }
            );
        }

        #[test]
        fn test_parse_highlight_and_white_and_message() {
            let actual = text_parse(String::from("abc white message"));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("abc"),
                    color: Some(String::from("white")),
                    message: Some(String::from("message")),
                    separator: Some(String::from(WS)),
                }
            );
        }

        #[test]
        fn test_parse_highlight_and_yellow_and_message() {
            let actual = text_parse(String::from("xyz yellow message"));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("xyz"),
                    color: Some(String::from("yellow")),
                    message: Some(String::from("message")),
                    separator: Some(String::from(" ")),
                }
            );
        }

        #[test]
        fn test_parse_with_some_messages() {
            let actual = text_parse(String::from("xyz white some messages."));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("xyz"),
                    color: Some(String::from("white")),
                    message: Some(String::from("some messages.")),
                    separator: Some(String::from(" ")),
                }
            );
        }

        #[test]
        fn test_parse_without_color() {
            let actual = text_parse(String::from("abc with some messages."));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("abc"),
                    color: None,
                    message: Some(String::from("with some messages.")),
                    separator: Some(String::from(" ")),
                }
            );
        }

        #[test]
        fn test_parse_highlight_and_yellow_and_message_with_linefeed() {
            let actual = text_parse(String::from("abc yellow\nmessage"));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("abc"),
                    color: Some(String::from("yellow")),
                    message: Some(String::from("message")),
                    separator: Some(String::from("\n")),
                }
            );
        }

        #[test]
        fn test_parse_highlight_message_with_linefeed() {
            let actual = text_parse(String::from("abc\nyellow yellow message\nsecond message"));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("abc"),
                    color: None,
                    message: Some(String::from("yellow yellow message\nsecond message")),
                    separator: Some(String::from("\n")),
                }
            );
        }

        #[test]
        fn test_parse_highlight_with_space_and_linefeed() {
            let actual = text_parse(String::from(
                "abc \n  yellow yellow message\nsecond message",
            ));
            assert_eq!(
                actual,
                ParsedText {
                    highlight: String::from("abc"),
                    color: None,
                    message: Some(String::from("yellow yellow message\nsecond message")),
                    separator: Some(String::from(" \n  ")),
                }
            );
        }
    }
}
