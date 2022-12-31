#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidChar,
    LackOfPair,
}