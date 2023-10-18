use crate::qwerty::*;
use crate::constants::*;
use crate::utils::*;
use crate::error::KorError;

mod traits;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq)]
pub struct KorChar {
    cho: Option<u16>,
    joong: Option<u16>,
    jong: Option<u16>
}

impl KorChar {
    pub fn from_char(c: char) -> Result<KorChar, KorError> {
        KorChar::from_u16(c as u16)
    }

    pub fn to_char(&self) -> char {
        char::from_u32(self.to_u16() as u32).unwrap()
    }

    /// "rhkr" -> KorChar(곽)
    pub fn from_qwerty(qwerty: &[u16]) -> Result<KorChar, KorError> {
        let kor = qwerty_to_kor(qwerty);

        if kor.len() == 1 {
            KorChar::from_u16(kor[0])
        }

        else {
            Err(KorError::TooManyChars(kor))
        }
    }

    /// KorChar(곽) -> "rhkr"
    pub fn to_qwerty(&self) -> Vec<u16> {
        kor_to_qwerty(&[self.to_u16()])
    }

    pub fn from_u16(c: u16) -> Result<KorChar, KorError> {
        if c <= 'ㅣ' as u16 {
            if c >= 'ㄱ' as u16 {
                if c < 'ㅏ' as u16 {
                    Ok(KorChar {
                        cho: Some(c),
                        joong: None,
                        jong: None
                    })
                }

                else {
                    Ok(KorChar {
                        cho: None,
                        joong: Some(c),
                        jong: None
                    })
                }
            }

            else {
                Err(KorError::InvalidHangul(c))
            }
        }

        else if c <= '힣' as u16 {
            if c >= '가' as u16 {
                let cho = ((c - 44032) / 588) as usize;
                let joong = ((c - 44032) % 588 / 28) as usize;
                let jong = ((c - 44032) % 588 % 28) as usize;

                Ok(KorChar::combine(
                    Some(CHOS[cho]),
                    Some(JOONGS[joong]),
                    if jong == 0 { None } else { Some(JONGS[jong - 1]) },
                ).unwrap())
            }

            else {
                Err(KorError::InvalidHangul(c))
            }
        }

        else {
            Err(KorError::InvalidHangul(c))
        }
    }

    pub fn to_u16(&self) -> u16 {
        if self.joong.is_none() {
            if self.cho.is_some() {
                self.cho.unwrap()
            }

            else if self.jong.is_some() {
                self.jong.unwrap()
            }

            else {
                '?' as u16
            }
        }

        else if self.cho.is_none() {
            self.joong.unwrap()
        }

        else if self.is_valid() {
            44032
            + rev_ind_cho(self.cho.unwrap()) * 588
            + rev_ind_joong(self.joong.unwrap()) * 28
            + rev_ind_jong(self.jong)
        }

        else {
            '?' as u16
        }
    }

    pub fn has_jong(&self) -> bool {
        self.jong.is_some()
    }

    /// 가 (O), ㄱ (X)
    pub fn is_valid(&self) -> bool {
        is_valid_cho(self.cho.unwrap())
        && is_valid_joong(self.joong.unwrap())
        && (self.jong.is_none() || is_valid_jong(self.jong.unwrap()))
    }

    /// Some('ㄱ'), Some('ㅣ'), None -> Ok(KorChar('기'))\
    /// Some('ㅂ'), Some('ㅏ'), Some('ㄱ') -> Ok(KorChar('박'))
    pub fn combine(cho: Option<u16>, joong: Option<u16>, jong: Option<u16>) -> Result<KorChar, KorError> {
        let result = KorChar {
            cho, joong, jong
        };

        if result.is_valid()
            || (cho.is_some() && (is_valid_cho(cho.unwrap()) || is_valid_jong(cho.unwrap())) && joong.is_none() && jong.is_none())
            || (joong.is_some() && is_valid_joong(joong.unwrap()) && cho.is_none() && jong.is_none())
        {
            Ok(result)
        }

        else {
            Err(result.find_error())
        }
    }

    pub fn set_cho(&self, c: u16) -> Result<Self, KorError> {
        if is_valid_cho(c) {
            let mut result = self.clone();
            result.cho = Some(c);
            Ok(result)
        }

        else {
            Err(KorError::InvalidCho(c))
        }
    }

    pub fn set_joong(&self, c: u16) -> Result<Self, KorError> {
        if is_valid_joong(c) {
            let mut result = self.clone();
            result.joong = Some(c);
            Ok(result)
        }

        else {
            Err(KorError::InvalidJoong(c))
        }
    }

    pub fn set_jong(&self, c: Option<u16>) -> Result<Self, KorError> {
        if c.is_none() || is_valid_jong(c.unwrap()) {
            let mut result = self.clone();
            result.jong = c;
            Ok(result)
        }

        else {
            Err(KorError::InvalidJong(c.unwrap()))
        }
    }

    /// (self.cho, self.joong, self.jong)
    pub fn disassemble(&self) -> (Option<u16>, Option<u16>, Option<u16>) {
        (self.cho, self.joong, self.jong)
    }

    fn find_error(&self) -> KorError {
        match self.cho {
            Some(c) if !is_valid_cho(c) => {
                return KorError::InvalidCho(c);
            },
            _ => {}
        }

        match self.joong {
            Some(c) if !is_valid_joong(c) => {
                return KorError::InvalidJoong(c);
            },
            _ => {}
        }

        match self.jong {
            Some(c) if !is_valid_jong(c) => {
                return KorError::InvalidJong(c);
            },
            _ => {}
        }

        if self.cho.is_none() {
            return KorError::MissingCho;
        }

        if self.joong.is_none() {
            return KorError::MissingJoong;
        }

        KorError::TODO
    }
}

/// (ㄱ, ㅅ) -> ㄳ\
/// (ㅡ, ㅣ) -> ㅢ
pub fn assemble(c1: u16, c2: u16) -> Option<u16> {

    /*
     * Sadly, Rust does not support `as` in patterns
     * 12593: ㄱ
     * 12596: ㄴ
     * 12599: ㄷ
     * 12601: ㄹ
     * 12609: ㅁ
     * 12610: ㅂ
     * 12613: ㅅ
     * 12616: ㅈ
     * 12620: ㅌ
     * 12621: ㅍ
     * 12622: ㅎ
     * 12623: ㅏ
     * 12624: ㅐ
     * 12627: ㅓ
     * 12628: ㅔ
     * 12631: ㅗ
     * 12636: ㅜ
     * 12641: ㅡ
     * 12643: ㅣ
     */
    match (c1, c2) {
        (12593, 12593) => Some('ㄲ' as u16),
        (12593, 12613) => Some('ㄳ' as u16),
        (12596, 12616) => Some('ㄵ' as u16),
        (12596, 12622) => Some('ㄶ' as u16),
        (12599, 12599) => Some('ㄸ' as u16),
        (12601, 12593) => Some('ㄺ' as u16),
        (12601, 12609) => Some('ㄻ' as u16),
        (12601, 12610) => Some('ㄼ' as u16),
        (12601, 12613) => Some('ㄽ' as u16),
        (12601, 12620) => Some('ㄾ' as u16),
        (12601, 12621) => Some('ㄿ' as u16),
        (12601, 12622) => Some('ㅀ' as u16),
        (12610, 12610) => Some('ㅃ' as u16),
        (12610, 12613) => Some('ㅄ' as u16),
        (12613, 12613) => Some('ㅆ' as u16),
        (12616, 12616) => Some('ㅉ' as u16),
        (12631, 12623) => Some('ㅘ' as u16),
        (12631, 12624) => Some('ㅙ' as u16),
        (12631, 12643) => Some('ㅚ' as u16),
        (12636, 12627) => Some('ㅝ' as u16),
        (12636, 12628) => Some('ㅞ' as u16),
        (12636, 12643) => Some('ㅟ' as u16),
        (12641, 12643) => Some('ㅢ' as u16),
        _ => None
    }
}

/// ㄳ -> (ㄱ, ㅅ)\
/// ㅢ -> (ㅡ, ㅣ)
pub fn disassemble(c: u16) -> Option<(u16, u16)> {

    /*
     * Sadly, Rust does not support `as` in patterns
     * 12594: ㄲ
     * 12595: ㄳ
     * 12597: ㄵ
     * 12598: ㄶ
     * 12600: ㄸ
     * 12602: ㄺ
     * 12603: ㄻ
     * 12604: ㄼ
     * 12605: ㄽ
     * 12606: ㄾ
     * 12607: ㄿ
     * 12608: ㅀ
     * 12611: ㅃ
     * 12612: ㅄ
     * 12614: ㅆ
     * 12617: ㅉ
     * 12632: ㅘ
     * 12633: ㅙ
     * 12634: ㅚ
     * 12637: ㅝ
     * 12638: ㅞ
     * 12639: ㅟ
     * 12642: ㅢ
     */
    match c {
        12594 => Some(('ㄱ' as u16, 'ㄱ' as u16)),
        12595 => Some(('ㄱ' as u16, 'ㅅ' as u16)),
        12597 => Some(('ㄴ' as u16, 'ㅈ' as u16)),
        12598 => Some(('ㄴ' as u16, 'ㅎ' as u16)),
        12600 => Some(('ㄷ' as u16, 'ㄷ' as u16)),
        12602 => Some(('ㄹ' as u16, 'ㄱ' as u16)),
        12603 => Some(('ㄹ' as u16, 'ㅁ' as u16)),
        12604 => Some(('ㄹ' as u16, 'ㅂ' as u16)),
        12605 => Some(('ㄹ' as u16, 'ㅅ' as u16)),
        12606 => Some(('ㄹ' as u16, 'ㅌ' as u16)),
        12607 => Some(('ㄹ' as u16, 'ㅍ' as u16)),
        12608 => Some(('ㄹ' as u16, 'ㅎ' as u16)),
        12611 => Some(('ㅂ' as u16, 'ㅂ' as u16)),
        12612 => Some(('ㅂ' as u16, 'ㅅ' as u16)),
        12614 => Some(('ㅅ' as u16, 'ㅅ' as u16)),
        12617 => Some(('ㅈ' as u16, 'ㅈ' as u16)),
        12632 => Some(('ㅗ' as u16, 'ㅏ' as u16)),
        12633 => Some(('ㅗ' as u16, 'ㅐ' as u16)),
        12634 => Some(('ㅗ' as u16, 'ㅣ' as u16)),
        12637 => Some(('ㅜ' as u16, 'ㅓ' as u16)),
        12638 => Some(('ㅜ' as u16, 'ㅔ' as u16)),
        12639 => Some(('ㅜ' as u16, 'ㅣ' as u16)),
        12642 => Some(('ㅡ' as u16, 'ㅣ' as u16)),
        _ => None
    }
}

// 'ㄱ' -> 0, 'ㄲ' -> 1, 'ㄴ' -> 2, ...
fn rev_ind_cho(c: u16) -> u16 {
    #[cfg(test)]
    assert!(is_valid_cho(c));

    REV_CHOS[c as usize - 'ㄱ' as usize]
}

// 'ㅏ' -> 0, 'ㅑ' -> 1, ...
fn rev_ind_joong(c: u16) -> u16 {
    #[cfg(test)]
    assert!(is_valid_joong(c));

    REV_JOONGS[c as usize - 'ㅏ' as usize]
}

// None -> 0, Some('ㄱ') -> 1, Some('ㄲ') -> 2, ...
fn rev_ind_jong(c: Option<u16>) -> u16 {
    #[cfg(test)]
    assert!(c.is_none() || is_valid_jong(c.unwrap()));

    if c.is_none() { 0 } else { REV_JONGS[c.unwrap() as usize - 'ㄱ' as usize] + 1 }
}
