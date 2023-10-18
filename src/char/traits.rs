use super::*;
use std::{cmp::Ordering, fmt};

impl fmt::Display for KorChar {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.to_char())
    }
}

impl PartialOrd for KorChar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_u16().partial_cmp(&other.to_u16())
    }
}

impl From<KorChar> for u16 {
    fn from(c: KorChar) -> u16 {
        c.to_u16()
    }
}

impl From<&KorChar> for u16 {
    fn from(c: &KorChar) -> u16 {
        c.to_u16()
    }
}

impl TryFrom<u16> for KorChar {
    type Error = KorError;

    fn try_from(n: u16) -> Result<KorChar, Self::Error> {
        KorChar::from_u16(n)
    }
}
