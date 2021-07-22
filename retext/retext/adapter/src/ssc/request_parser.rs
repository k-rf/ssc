#[derive(Debug, PartialEq)]
pub struct ParsedRequestText {
    pub highlight: String,
    pub color: Option<String>,
    pub message: Option<String>,
}

pub fn text_parse(text: String) -> ParsedRequestText {
    let mut s = text.split_inclusive(' ');

    let highlight = String::from(s.next().expect("Highlight text is necessary"));

    let mut h = highlight.split_inclusive('\n');
    let highlight = String::from(h.next().unwrap());

    if let Some(msg) = h.next() {
        return ParsedRequestText {
            highlight,
            color: None,
            message: Some(String::from(msg) + &s.collect::<String>()),
        };
    }

    let color = String::from(s.next().unwrap_or_default()).to_lowercase();

    let (color, msg) = match color.trim() {
        "white" => (Some(String::from("white")), None),
        "yellow" => (Some(String::from("yellow")), None),
        "" => (None, None),
        _ => (None, Some(color)),
    };

    let message = s.collect::<String>();

    let message = if let Some(msg) = msg {
        Some(msg + &message)
    } else {
        if message.is_empty() {
            None
        } else {
            Some(message)
        }
    };

    ParsedRequestText {
        highlight,
        color,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_second_arg_yellow() {
        let actual = text_parse(String::from("abc yellow message"));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("abc "),
                color: Some(String::from("yellow")),
                message: Some(String::from("message"))
            }
        );
    }

    #[test]
    fn test_parse_second_arg_white() {
        let actual = text_parse(String::from("xyz white msg"));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("xyz "),
                color: Some(String::from("white")),
                message: Some(String::from("msg"))
            }
        );
    }

    #[test]
    fn test_parse_with_some_messages() {
        let actual = text_parse(String::from("xyz white some messages."));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("xyz "),
                color: Some(String::from("white")),
                message: Some(String::from("some messages."))
            }
        );
    }

    #[test]
    fn test_parse_without_color() {
        let actual = text_parse(String::from("abc with some messages."));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("abc "),
                color: None,
                message: Some(String::from("with some messages."))
            }
        );
    }

    #[test]
    fn test_parse_without_message() {
        let actual = text_parse(String::from("abc white"));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("abc "),
                color: Some(String::from("white")),
                message: None
            }
        );
    }

    #[test]
    fn test_parse_only_highlight() {
        let actual = text_parse(String::from("abc"));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("abc"),
                color: None,
                message: None,
            }
        );
    }

    #[test]
    fn test_parse_highlight_after_linefeed() {
        let actual = text_parse(String::from("abc\nyellow message"));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("abc\n"),
                color: None,
                message: Some(String::from("yellow message")),
            }
        );
    }

    #[test]
    fn test_parse_highlight_after_linefeed_after_color() {
        let actual = text_parse(String::from("abc\nyellow yellow message\nsecond message"));
        assert_eq!(
            actual,
            ParsedRequestText {
                highlight: String::from("abc\n"),
                color: None,
                message: Some(String::from("yellow yellow message\nsecond message")),
            }
        );
    }
}
