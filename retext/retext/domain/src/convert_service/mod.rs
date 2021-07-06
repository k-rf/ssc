use crate::emoji::emoji_alphabet::Color;
use crate::emoji::emoji_text::EmojiText;
use crate::raw::raw_text::RawText;

pub fn convert(src: RawText, color: Color) -> EmojiText {
    EmojiText::from(
        src.get()
            .into_iter()
            .map(|v| v.get().to_string())
            .collect::<String>()
            .as_str(),
        color,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_white() {
        let result = convert(RawText::from("abc"), Color::White);

        for (emoji, expected) in result.get().iter().zip("abc".chars()) {
            assert_eq!(emoji.get(), format!(":alphabet-white-{}:", expected))
        }
    }

    #[test]
    fn test_convert_to_yellow() {
        let result = convert(RawText::from("abc"), Color::Yellow);

        for (emoji, expected) in result.get().iter().zip("abc".chars()) {
            assert_eq!(emoji.get(), format!(":alphabet-yellow-{}:", expected))
        }
    }
}
