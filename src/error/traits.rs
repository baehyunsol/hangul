use super::*;
use crate::utils::from_v16;
use std::fmt;

impl fmt::Display for KorError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt, "{}",
            match self {
                KorError::MissingCho => "a cho-sung is expected, but got nothing".to_string(),
                KorError::MissingJoong => "a joong-sung is expected, but got nothing".to_string(),
                KorError::InvalidCho(c) => format!("{} is not a valid cho-sung", try_convert(*c)),
                KorError::InvalidJoong(c) => format!("{} is not a valid joong-sung", try_convert(*c)),
                KorError::InvalidJong(c) => format!("{} is not a valid jong-sung", try_convert(*c)),
                KorError::InvalidHangul(c) => format!("{} is not a valid hangul", try_convert(*c)),
                KorError::TooManyChars(s) => format!("expected one character, but got too many: {:?}", from_v16(s)),
                KorError::TODO => format!("There's an error, but I'm too lazy to impl a variant for that..."),
            },
        )
    }
}

fn try_convert(c: u16) -> String {
    match char::from_u32(c as u32) {
        Some(c) => format!("{c:?}"),
        None => format!("\"\\{:x}\"", c),
    }
}
