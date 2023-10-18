use crate::utils::to_lower;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

pub const CHOS: [u16; 19] = [
    'ㄱ' as u16, 'ㄲ' as u16, 'ㄴ' as u16, 'ㄷ' as u16, 'ㄸ' as u16,
    'ㄹ' as u16, 'ㅁ' as u16, 'ㅂ' as u16, 'ㅃ' as u16, 'ㅅ' as u16,
    'ㅆ' as u16, 'ㅇ' as u16, 'ㅈ' as u16, 'ㅉ' as u16, 'ㅊ' as u16,
    'ㅋ' as u16, 'ㅌ' as u16, 'ㅍ' as u16, 'ㅎ' as u16
];

pub const JOONGS: [u16; 21] = [
    'ㅏ' as u16, 'ㅐ' as u16, 'ㅑ' as u16, 'ㅒ' as u16, 'ㅓ' as u16,
    'ㅔ' as u16, 'ㅕ' as u16, 'ㅖ' as u16, 'ㅗ' as u16, 'ㅘ' as u16,
    'ㅙ' as u16, 'ㅚ' as u16, 'ㅛ' as u16, 'ㅜ' as u16, 'ㅝ' as u16,
    'ㅞ' as u16, 'ㅟ' as u16, 'ㅠ' as u16, 'ㅡ' as u16, 'ㅢ' as u16, 'ㅣ' as u16
];

pub const JONGS: [u16; 27] = [
    'ㄱ' as u16, 'ㄲ' as u16, 'ㄳ' as u16, 'ㄴ' as u16, 'ㄵ' as u16,
    'ㄶ' as u16, 'ㄷ' as u16, 'ㄹ' as u16, 'ㄺ' as u16, 'ㄻ' as u16,
    'ㄼ' as u16, 'ㄽ' as u16, 'ㄾ' as u16, 'ㄿ' as u16, 'ㅀ' as u16,
    'ㅁ' as u16, 'ㅂ' as u16, 'ㅄ' as u16, 'ㅅ' as u16, 'ㅆ' as u16,
    'ㅇ' as u16, 'ㅈ' as u16, 'ㅊ' as u16, 'ㅋ' as u16, 'ㅌ' as u16,
    'ㅍ' as u16, 'ㅎ' as u16
];

pub const CONSONANTS: [u16; 30] = [
    'ㄱ' as u16, 'ㄲ' as u16, 'ㄳ' as u16, 'ㄴ' as u16, 'ㄵ' as u16,
    'ㄶ' as u16, 'ㄷ' as u16, 'ㄸ' as u16, 'ㄹ' as u16, 'ㄺ' as u16,
    'ㄻ' as u16, 'ㄼ' as u16, 'ㄽ' as u16, 'ㄾ' as u16, 'ㄿ' as u16,
    'ㅀ' as u16, 'ㅁ' as u16, 'ㅂ' as u16, 'ㅃ' as u16, 'ㅄ' as u16,
    'ㅅ' as u16, 'ㅆ' as u16, 'ㅇ' as u16, 'ㅈ' as u16, 'ㅉ' as u16,
    'ㅊ' as u16, 'ㅋ' as u16, 'ㅌ' as u16, 'ㅍ' as u16, 'ㅎ' as u16
];

pub const VOWELS: [u16; 21] = JOONGS;

lazy_static! {
    pub(crate) static ref REV_CHOS: Vec<u16> = {
        let mut r = vec![0;30];

        for (ind, c) in CHOS.iter().enumerate() {
            r[*c as usize - 'ㄱ' as usize] = ind as u16;
        }

        r
    };

    pub(crate) static ref REV_JOONGS: Vec<u16> = {
        let mut r = vec![0;30];

        for (ind, c) in JOONGS.iter().enumerate() {
            r[*c as usize - 'ㅏ' as usize] = ind as u16;
        }

        r
    };

    pub(crate) static ref REV_JONGS: Vec<u16> = {
        let mut r = vec![0;30];

        for (ind, c) in JONGS.iter().enumerate() {
            r[*c as usize - 'ㄱ' as usize] = ind as u16;
        }

        r
    };

    pub(crate) static ref QWERTY_TO_KOR: HashMap<u16, u16> = {
        let mut result = HashMap::with_capacity(52);

        result.insert('q' as u16, 'ㅂ' as u16); result.insert('Q' as u16, 'ㅃ' as u16);
        result.insert('w' as u16, 'ㅈ' as u16); result.insert('W' as u16, 'ㅉ' as u16);
        result.insert('e' as u16, 'ㄷ' as u16); result.insert('E' as u16, 'ㄸ' as u16);
        result.insert('r' as u16, 'ㄱ' as u16); result.insert('R' as u16, 'ㄲ' as u16);
        result.insert('t' as u16, 'ㅅ' as u16); result.insert('T' as u16, 'ㅆ' as u16);
        result.insert('y' as u16, 'ㅛ' as u16); result.insert('Y' as u16, 'ㅛ' as u16);
        result.insert('u' as u16, 'ㅕ' as u16); result.insert('U' as u16, 'ㅕ' as u16);
        result.insert('i' as u16, 'ㅑ' as u16); result.insert('I' as u16, 'ㅑ' as u16);
        result.insert('o' as u16, 'ㅐ' as u16); result.insert('O' as u16, 'ㅒ' as u16);
        result.insert('p' as u16, 'ㅔ' as u16); result.insert('P' as u16, 'ㅖ' as u16);
        result.insert('a' as u16, 'ㅁ' as u16); result.insert('A' as u16, 'ㅁ' as u16);
        result.insert('s' as u16, 'ㄴ' as u16); result.insert('S' as u16, 'ㄴ' as u16);
        result.insert('d' as u16, 'ㅇ' as u16); result.insert('D' as u16, 'ㅇ' as u16);
        result.insert('f' as u16, 'ㄹ' as u16); result.insert('F' as u16, 'ㄹ' as u16);
        result.insert('g' as u16, 'ㅎ' as u16); result.insert('G' as u16, 'ㅎ' as u16);
        result.insert('h' as u16, 'ㅗ' as u16); result.insert('H' as u16, 'ㅗ' as u16);
        result.insert('j' as u16, 'ㅓ' as u16); result.insert('J' as u16, 'ㅓ' as u16);
        result.insert('k' as u16, 'ㅏ' as u16); result.insert('K' as u16, 'ㅏ' as u16);
        result.insert('l' as u16, 'ㅣ' as u16); result.insert('L' as u16, 'ㅣ' as u16);
        result.insert('z' as u16, 'ㅋ' as u16); result.insert('Z' as u16, 'ㅋ' as u16);
        result.insert('x' as u16, 'ㅌ' as u16); result.insert('X' as u16, 'ㅌ' as u16);
        result.insert('c' as u16, 'ㅊ' as u16); result.insert('C' as u16, 'ㅊ' as u16);
        result.insert('v' as u16, 'ㅍ' as u16); result.insert('V' as u16, 'ㅍ' as u16);
        result.insert('b' as u16, 'ㅠ' as u16); result.insert('B' as u16, 'ㅠ' as u16);
        result.insert('n' as u16, 'ㅜ' as u16); result.insert('N' as u16, 'ㅜ' as u16);
        result.insert('m' as u16, 'ㅡ' as u16); result.insert('M' as u16, 'ㅡ' as u16);

        assert_eq!(result.len(), 52);
        assert_eq!(result.values().collect::<HashSet<&u16>>().len(), 33);

        result
    };

    pub(crate) static ref KOR_TO_QWERTY: HashMap<u16, u16> = {
        let mut result = HashMap::with_capacity(QWERTY_TO_KOR.len());

        for (qwerty, korean) in QWERTY_TO_KOR.iter() {
            if result.contains_key(korean) {
                result.insert(*korean, to_lower(qwerty));
            }

            else {
                result.insert(*korean, *qwerty);
            }
        }

        result
    };
}
