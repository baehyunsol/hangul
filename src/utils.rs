#[inline]
pub fn to_lower(c: &u16) -> u16 {
    if 64 < *c && *c < 91 {
        *c + 32
    }

    else {
        *c
    }
}

/// ㄲㄸㅃㅆㅉ
#[inline]
pub fn is_tensory(c: u16) -> bool {
    c == 'ㄲ' as u16 ||
    c == 'ㄸ' as u16 ||
    c == 'ㅃ' as u16 ||
    c == 'ㅆ' as u16 ||
    c == 'ㅉ' as u16
}

/// ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ\
/// [12593, 12594, 12596, 12599, 12600, 12601, 12609, 12610, 12611, 12613, 12614, 12615, 12616, 12617, 12618, 12619, 12620, 12621, 12622]
#[inline]
pub fn is_valid_cho(c: u16) -> bool {
    12592 < c && c < 12623 && (
        c != 12595 && c != 12597 && c != 12598
        && !(12601 < c && c < 12609) && c != 12612
    )
}

/// ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ\
/// 12623~12643
#[inline]
pub fn is_valid_joong(c: u16) -> bool {
    12622 < c && c < 12644
}

/// ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ\
/// [12593, 12594, 12595, 12596, 12597, 12598, 12599, 12601, 12602, 12603, 12604, 12605, 12606, 12607, 12608, 12609, 12610, 12612, 12613, 12614, 12615, 12616, 12618, 12619, 12620, 12621, 12622]
#[inline]
pub fn is_valid_jong(c: u16) -> bool {
    12592 < c && c < 12623 && (
        c != 12600 && c != 12611 && c != 12617
    )
}

#[inline]
pub fn is_hangul(c: u16) -> bool {
    is_jamo(c) || is_non_jamo(c)
}

#[inline]
pub fn is_jamo(c: u16) -> bool {
    44031 < c && c < 55204
}

#[inline]
pub fn is_non_jamo(c: u16) -> bool {
    12592 < c && c < 12644
}

#[inline]
pub fn is_valid_consonant(c: u16) -> bool {
    12592 < c && c < 12623
}

#[inline]
pub fn is_valid_vowel(c: u16) -> bool {
    is_valid_joong(c)
}

#[inline]
pub fn from_v16(v: &[u16]) -> String {
    String::from_utf16(v).unwrap()
}

#[inline]
pub fn into_v16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}
