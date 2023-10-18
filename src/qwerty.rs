use crate::constants::*;
use crate::*;

enum ParseState {
    None,
    Cho(u16),
    Joong(u16, u16),
    Jong(u16, u16, u16)
}

/// qogusthf -> 배현솔
pub fn qwerty_to_kor(string: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(string.len());
    let mut curr_parse_state = ParseState::None;
    let mut index = 0;

    while index <= string.len() {
        let q = if index < string.len() {
            string[index]
        } else {
            ' ' as u16  // it handles the last hangul
        };
        index += 1;

        let next_char = match QWERTY_TO_KOR.get(&q) {
            Some(k) => k,
            _ => &q
        };

        match curr_parse_state {
            ParseState::None => {
                if is_valid_consonant(*next_char) {
                    curr_parse_state = ParseState::Cho(*next_char);
                }

                // ml -> ㅢ
                else if is_valid_vowel(*next_char) && index > 1 && is_valid_vowel(result[result.len() - 1]) {
                    match assemble(result[result.len() - 1], *next_char) {
                        Some(v) => {
                            result.pop();
                            result.push(v);
                        }
                        _ => {
                            result.push(*next_char);
                        }
                    }
                }

                else {
                    result.push(*next_char);
                }
            }
            ParseState::Cho(c) => {
                if is_valid_consonant(*next_char) {
                    match assemble(c, *next_char) {
                        // `ㄱㄱ` is `rr`, and `ㄲ` is `R` -> that means `rr` is not `ㄲ`
                        Some(new_c) if !is_tensory(new_c) => {
                            curr_parse_state = ParseState::Cho(new_c);
                        }
                        _ => {
                            result.push(c);
                            curr_parse_state = ParseState::Cho(*next_char);
                        }
                    }
                }

                else if is_valid_vowel(*next_char) {
                    curr_parse_state = ParseState::Joong(c, *next_char);
                }

                else {
                    result.push(c);
                    result.push(*next_char);
                    curr_parse_state = ParseState::None;
                }
            }
            ParseState::Joong(c, j) => {
                if is_valid_consonant(*next_char) {
                    curr_parse_state = ParseState::Jong(c, j, *next_char);
                }

                else if is_valid_vowel(*next_char) {
                    match assemble(j, *next_char) {
                        Some(new_v) => {
                            curr_parse_state = ParseState::Joong(c, new_v);
                        },
                        _ => {
                            match KorChar::combine(Some(c), Some(j), None) {
                                Ok(h) => {
                                    result.push(h.to_u16());
                                }
                                _ => {
                                    result.push(c);
                                    result.push(j);
                                }
                            }

                            result.push(*next_char);
                            curr_parse_state = ParseState::None;
                        }
                    }
                }

                else {
                    match KorChar::combine(Some(c), Some(j), None) {
                        Ok(h) => {
                            result.push(h.to_u16());
                        }
                        _ => {
                            let (c1, c2) = disassemble(c).unwrap();
                            let h = KorChar::combine(Some(c2), Some(j), None).unwrap();

                            result.push(c1);
                            result.push(h.to_u16());
                        }
                    }

                    result.push(*next_char);
                    curr_parse_state = ParseState::None;
                }

            }
            ParseState::Jong(c, ju, jo) => {
                if is_valid_consonant(*next_char) {
                    match assemble(jo, *next_char) {
                        Some(new_c) if !is_tensory(new_c) => {
                            curr_parse_state = ParseState::Jong(c, ju, new_c);
                        }
                        _ => {
                            match KorChar::combine(Some(c), Some(ju), Some(jo)) {
                                Ok(h) => {
                                    result.push(h.to_u16());
                                }
                                _ => match KorChar::combine(Some(c), Some(ju), None) {
                                    Ok(h) => {
                                        result.push(h.to_u16());
                                        result.push(jo);
                                    }
                                    _ => {
                                        result.push(c);
                                        result.push(ju);
                                        result.push(jo);
                                    }
                                }
                            }

                            curr_parse_state = ParseState::Cho(*next_char);
                        }
                    }
                }

                else if is_valid_vowel(*next_char) {
                    match disassemble(jo) {
                        Some((c1, c2)) if !is_tensory(jo) => {

                            match KorChar::combine(Some(c), Some(ju), Some(c1)) {
                                Ok(h) => {
                                    result.push(h.to_u16());
                                }
                                // ( c )    ju    c1    c2
                                // ㄱ ㅅ    ㅣ    ㄱ     ㅅ
                                _ => {
                                    let (c3, c4) = disassemble(c).unwrap();
                                    let h = KorChar::combine(Some(c4), Some(ju), Some(c1)).unwrap();

                                    result.push(c3);
                                    result.push(h.to_u16());
                                }
                            }

                            curr_parse_state = ParseState::Joong(c2, *next_char);
                        }
                        _ => {
                            match KorChar::combine(Some(c), Some(ju), None) {
                                Ok(h) => {
                                    result.push(h.to_u16());
                                }
                                // ( c )    ju    jo
                                // ㄱ ㅅ    ㅣ    ㄷ
                                _ => {
                                    let (c1, c2) = disassemble(c).unwrap();
                                    let h = KorChar::combine(Some(c2), Some(ju), None).unwrap();

                                    result.push(c1);
                                    result.push(h.to_u16());
                                }
                            }

                            curr_parse_state = ParseState::Joong(jo, *next_char);
                        }
                    }
                }

                else {
                    match KorChar::combine(Some(c), Some(ju), Some(jo)) {
                        Ok(h) => {
                            result.push(h.to_u16());
                        }
                        /*
                         *          ㅆ      ㄸ       ㄳ       ㄱ
                         *  ㅆ      쌌      땄      ㄱ샀      갔
                         *  ㄸ      싸ㄸ    따ㄸ     ㄱ사ㄸ    가ㄸ
                         *  ㄳ      싻      딳       ㄱ삯     갃
                         *  ㄱ      싹      딱       ㄱ삭     각
                         */
                        _ => if is_tensory(jo) {
                            match KorChar::combine(Some(c), Some(ju), None) {
                                Ok(h) => {
                                    result.push(h.to_u16());
                                    result.push(jo);
                                }
                                _ => {
                                    let (c1, c2) = disassemble(c).unwrap();
                                    let h = KorChar::combine(Some(c2), Some(ju), None).unwrap();

                                    result.push(c1);
                                    result.push(h.to_u16());
                                    result.push(jo);
                                }
                            }

                        } else {
                            let (c1, c2) = disassemble(c).unwrap();
                            let h = KorChar::combine(Some(c2), Some(ju), Some(jo)).unwrap();

                            result.push(c1);
                            result.push(h.to_u16());
                        }
                    }

                    result.push(*next_char);
                    curr_parse_state = ParseState::None;
                }
            }
        }
    }

    // pops the extra whitespace
    result.pop().unwrap();

    result
}

/// 배현솔 -> qogusthf
pub fn kor_to_qwerty(string: &[u16]) -> Vec<u16> {
    let mut result = Vec::with_capacity(string.len() * 3);

    for c in string.iter() {
        if is_non_jamo(*c) {
            match KOR_TO_QWERTY.get(c) {
                Some(q) => {
                    result.push(*q);
                }
                _ => match disassemble(*c) {
                    Some((c1, c2)) => {
                        result.push(*KOR_TO_QWERTY.get(&c1).unwrap());
                        result.push(*KOR_TO_QWERTY.get(&c2).unwrap());
                    },
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        else if is_jamo(*c) {
            let (cho, joong, jong) = KorChar::from_u16(*c).unwrap().disassemble();

            for c in [cho, joong, jong] {
                match c {
                    Some(c_) => match KOR_TO_QWERTY.get(&c_) {
                        Some(q) => {
                            result.push(*q);
                        }
                        _ => match disassemble(c_) {
                            Some((c1, c2)) => {
                                result.push(*KOR_TO_QWERTY.get(&c1).unwrap());
                                result.push(*KOR_TO_QWERTY.get(&c2).unwrap());
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        else {
            result.push(*c);
        }
    }

    result
}
