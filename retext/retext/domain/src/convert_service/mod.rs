use crate::emoji::{emoji_alphabet::EmojiAlphabet, emoji_text::EmojiText};
use crate::raw::raw_text::RawText;

pub fn convert(src: RawText) -> EmojiText {
    EmojiText::new(
        src.get()
            .into_iter()
            .map(|v| EmojiAlphabet::new(v.get()))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_white() {
        let result = convert(RawText::from("abc"));
        for (emoji, expected) in result.get().iter().zip("abc".chars()) {
            assert_eq!(emoji.get_white(), format!(":alphabet-white-{}:", expected))
        }
    }

    #[test]
    fn test_convert_to_yellow() {
        let result = convert(RawText::from("abc"));
        for (emoji, expected) in result.get().iter().zip("abc".chars()) {
            assert_eq!(
                emoji.get_yellow(),
                format!(":alphabet-yellow-{}:", expected)
            )
        }
    }
}
