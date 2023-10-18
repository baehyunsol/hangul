mod traits;

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum KorError {
    InvalidCho(u16),
    InvalidJoong(u16),
    InvalidJong(u16),
    InvalidHangul(u16),
    TooManyChars(Vec<u16>),
    MissingCho,
    MissingJoong,
    TODO,
}
