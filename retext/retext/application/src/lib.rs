use domain::convert_service;
use domain::emoji::emoji_alphabet::Color as EmojiColor;
use domain::raw::raw_text::RawText;

#[derive(Debug)]
pub enum Color {
    White,
    Yellow,
}

#[derive(Debug)]
pub struct InputData {
    pub raw_text: String,
    pub color: Color,
}

#[derive(Debug, PartialEq)]
pub struct OutputData {
    pub emoji_text: String,
}

pub fn raw2emoji(input: InputData) -> OutputData {
    let raw_text = RawText::from(input.raw_text.as_str());
    let color = match input.color {
        Color::White => EmojiColor::White,
        Color::Yellow => EmojiColor::Yellow,
    };

    let result = convert_service::convert(raw_text, color);

    OutputData {
        emoji_text: result.get().iter().map(|e| e.get()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_white() {
        let actual = raw2emoji(InputData {
            raw_text: String::from("abc"),
            color: Color::White,
        });
        let expected = OutputData {
            emoji_text: String::from(
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
        let actual = raw2emoji(InputData {
            raw_text: String::from("xyz"),
            color: Color::Yellow,
        });
        let expected = OutputData {
            emoji_text: String::from(
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
}
