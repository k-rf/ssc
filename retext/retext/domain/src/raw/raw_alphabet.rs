#[derive(Debug, PartialEq, PartialOrd)]
pub struct RawAlphabet {
    value: char,
}

const USABLE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!?#@";

impl RawAlphabet {
    pub fn new(value: char) -> Result<Self, String> {
        if USABLE.contains(value) {
            Ok(RawAlphabet { value })
        } else {
            Err(String::from(format!("\"{}\" is not usable.", value)))
        }
    }

    pub fn get(self) -> char {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usable_characters() {
        for (actual, expected) in USABLE
            .chars()
            .map(|c| Box::new(RawAlphabet::new(c).unwrap()))
            .zip(USABLE.chars())
        {
            assert_eq!(actual.get(), expected);
        }
    }

    #[test]
    fn test_unusable_characters() {
        if let Err(s) = RawAlphabet::new('$') {
            assert_eq!(s, "\"$\" is not usable.");
        }
    }

    #[test]
    fn test_equal() {
        let a = RawAlphabet::new('a');
        let b = RawAlphabet::new('a');

        assert_eq!(a, b);
    }

    #[test]
    fn test_not_equal() {
        let a = RawAlphabet::new('a');
        let b = RawAlphabet::new('b');

        assert_ne!(a, b);
    }
}
