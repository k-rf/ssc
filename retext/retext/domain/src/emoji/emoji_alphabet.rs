const DECORATION_WHITE: &str = "alphabet-white-";
const DECORATION_YELLOW: &str = "alphabet-yellow-";

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Color {
    White,
    Yellow,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct EmojiAlphabet {
    value: char,
    color: Color,
}

impl EmojiAlphabet {
    pub fn new(value: char, color: Color) -> Self {
        EmojiAlphabet {
            value: value.to_ascii_lowercase(),
            color,
        }
    }

    pub fn get(&self) -> String {
        match self.color {
            Color::White => self.get_white(),
            Color::Yellow => self.get_yellow(),
        }
    }

    fn get_white(&self) -> String {
        format!(":{}{}:", DECORATION_WHITE, Self::to_string(&self.value))
    }

    fn get_yellow(&self) -> String {
        format!(":{}{}:", DECORATION_YELLOW, Self::to_string(&self.value))
    }

    fn to_string(c: &char) -> String {
        match Self::char_map(c) {
            Some(s) => String::from(s),
            None => c.to_string(),
        }
    }

    fn char_map<'a>(c: &char) -> Option<&'a str> {
        match c {
            '?' => Some("question"),
            '!' => Some("exclamation"),
            '#' => Some("hash"),
            '@' => Some("at"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let a = EmojiAlphabet::new('a', Color::White);
        let b = EmojiAlphabet::new('A', Color::White);
        assert_eq!(a, b);
    }

    #[test]
    fn test_not_equal_value() {
        let a = EmojiAlphabet::new('a', Color::White);
        let b = EmojiAlphabet::new('B', Color::White);

        assert_ne!(a, b);
    }

    #[test]
    fn test_not_equal_color() {
        let a = EmojiAlphabet::new('a', Color::White);
        let b = EmojiAlphabet::new('a', Color::Yellow);

        assert_ne!(a, b);
    }

    #[test]
    fn test_symbol() {
        assert_eq!(EmojiAlphabet::to_string(&'!'), "exclamation");
        assert_eq!(EmojiAlphabet::to_string(&'?'), "question");
        assert_eq!(EmojiAlphabet::to_string(&'#'), "hash");
        assert_eq!(EmojiAlphabet::to_string(&'@'), "at");
    }

    #[test]
    fn test_non_symbol() {
        assert_eq!(EmojiAlphabet::to_string(&'a'), "a");
    }

    #[test]
    fn test_get_white() {
        let actual = EmojiAlphabet::new('a', Color::White);
        assert_eq!(actual.get(), ":alphabet-white-a:");
    }

    #[test]
    fn test_get_yellow() {
        let actual = EmojiAlphabet::new('a', Color::Yellow);
        assert_eq!(actual.get(), ":alphabet-yellow-a:");
    }
}
