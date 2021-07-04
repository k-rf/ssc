const DECORATION_WHITE: &str = "alphabet-white-";
const DECORATION_YELLOW: &str = "alphabet-yellow-";

#[derive(Debug, PartialEq, PartialOrd)]
pub struct EmojiAlphabet {
    value: char,
}

impl EmojiAlphabet {
    pub fn new(value: char) -> Self {
        EmojiAlphabet {
            value: value.to_ascii_lowercase(),
        }
    }

    pub fn get_white(&self) -> String {
        format!(":{}{}:", DECORATION_WHITE, Self::to_string(&self.value))
    }

    pub fn get_yellow(&self) -> String {
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
        let a = EmojiAlphabet::new('a');
        let b = EmojiAlphabet::new('A');

        assert_eq!(a, b);
    }

    #[test]
    fn test_not_equal() {
        let a = EmojiAlphabet::new('a');
        let b = EmojiAlphabet::new('B');

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
}
