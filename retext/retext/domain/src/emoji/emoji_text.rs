use super::emoji_alphabet::EmojiAlphabet;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct EmojiText {
    value: Vec<EmojiAlphabet>,
}

impl EmojiText {
    pub fn new(value: Vec<EmojiAlphabet>) -> EmojiText {
        EmojiText { value }
    }

    pub fn get(&self) -> &Vec<EmojiAlphabet> {
        &self.value
    }
}
