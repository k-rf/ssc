use super::emoji_alphabet::{Color, EmojiAlphabet};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct EmojiText {
    value: Vec<EmojiAlphabet>,
}

impl EmojiText {
    pub fn new(value: Vec<EmojiAlphabet>) -> EmojiText {
        EmojiText { value }
    }

    pub fn from(value: &str, color: Color) -> EmojiText {
        EmojiText {
            value: value
                .chars()
                .into_iter()
                .map(|c| EmojiAlphabet::new(c, color))
                .collect(),
        }
    }

    pub fn get(&self) -> &Vec<EmojiAlphabet> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let actual = EmojiText::from("abc", Color::White);
        let actual = actual.get();
        let expected = vec![
            EmojiAlphabet::new('a', Color::White),
            EmojiAlphabet::new('b', Color::White),
            EmojiAlphabet::new('c', Color::White),
        ];

        assert_eq!(actual, &expected);
    }
}
