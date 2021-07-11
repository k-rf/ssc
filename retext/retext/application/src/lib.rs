use domain::convert_service;
pub use domain::emoji::emoji_alphabet::Color;
use domain::raw::raw_text::RawText;

#[derive(Debug)]
pub struct InputData<'a> {
    pub highlight: String,
    pub color: Option<&'a str>,
    pub message: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct OutputData {
    pub highlighted_message: String,
}

pub fn highlight_message(input: InputData) -> OutputData {
    let highlight = RawText::from(input.highlight.as_str());
    let color = match input.color {
        Some(color) => match color.to_lowercase().as_str() {
            "yellow" => Color::Yellow,
            _ => Color::White,
        },
        None => Color::White,
    };
    let message = input.message.unwrap_or_default();

    let result = convert_service::convert(highlight, color);

    OutputData {
        highlighted_message: result.get().iter().map(|e| e.get()).collect::<String>() + &message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_white() {
        let actual = highlight_message(InputData {
            highlight: String::from("abc"),
            color: Some("white"),
            message: None,
        });
        let expected = OutputData {
            highlighted_message: String::from(
                [
                    ":alphabet-white-a:",
                    ":alphabet-white-b:",
                    ":alphabet-white-c:",
                ]
                .join(""),
            ),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_to_yellow() {
        let actual = highlight_message(InputData {
            highlight: String::from("xyz"),
            color: Some("YeLLow"),
            message: None,
        });
        let expected = OutputData {
            highlighted_message: String::from(
                [
                    ":alphabet-yellow-x:",
                    ":alphabet-yellow-y:",
                    ":alphabet-yellow-z:",
                ]
                .join(""),
            ),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_with_message() {
        let actual = highlight_message(InputData {
            highlight: String::from("xyz"),
            color: None,
            message: Some("test message"),
        });
        let expected = OutputData {
            highlighted_message: String::from(
                [
                    ":alphabet-white-x:",
                    ":alphabet-white-y:",
                    ":alphabet-white-z:",
                ]
                .join(""),
            ) + "test message",
        };
        assert_eq!(actual, expected);
    }
}
