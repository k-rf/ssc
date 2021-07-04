use super::raw_alphabet::RawAlphabet;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RawText {
    value: Vec<RawAlphabet>,
}

impl RawText {
    pub fn new(value: Vec<RawAlphabet>) -> RawText {
        RawText { value }
    }

    pub fn from(value: &str) -> RawText {
        RawText {
            value: value
                .chars()
                .into_iter()
                .map(|c| RawAlphabet::new(c).unwrap())
                .collect(),
        }
    }

    pub fn get(self) -> Vec<RawAlphabet> {
        self.value
    }
}
