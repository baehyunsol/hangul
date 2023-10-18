use crate::*;
use crate::constants::*;

#[test]
fn combine_test() {
    let samples = vec![
        ('배' as u16, (Some('ㅂ' as u16), Some('ㅐ' as u16), None)),
        ('현' as u16, (Some('ㅎ' as u16), Some('ㅕ' as u16), Some('ㄴ' as u16))),
        ('솔' as u16, (Some('ㅅ' as u16), Some('ㅗ' as u16), Some('ㄹ' as u16))),
        ('서' as u16, (Some('ㅅ' as u16), Some('ㅓ' as u16), None)),
        ('울' as u16, (Some('ㅇ' as u16), Some('ㅜ' as u16), Some('ㄹ' as u16))),
        ('대' as u16, (Some('ㄷ' as u16), Some('ㅐ' as u16), None)),
        ('학' as u16, (Some('ㅎ' as u16), Some('ㅏ' as u16), Some('ㄱ' as u16))),
        ('교' as u16, (Some('ㄱ' as u16), Some('ㅛ' as u16), None)),
        ('예' as u16, (Some('ㅇ' as u16), Some('ㅖ' as u16), None)),
        ('비' as u16, (Some('ㅂ' as u16), Some('ㅣ' as u16), None)),
        ('군' as u16, (Some('ㄱ' as u16), Some('ㅜ' as u16), Some('ㄴ' as u16))),
        ('귀' as u16, (Some('ㄱ' as u16), Some('ㅟ' as u16), None)),
        ('찮' as u16, (Some('ㅊ' as u16), Some('ㅏ' as u16), Some('ㄶ' as u16))),
        ('아' as u16, (Some('ㅇ' as u16), Some('ㅏ' as u16), None)),
        ('값' as u16, (Some('ㄱ' as u16), Some('ㅏ' as u16), Some('ㅄ' as u16))),
        ('뷁' as u16, (Some('ㅂ' as u16), Some('ㅞ' as u16), Some('ㄺ' as u16))),
        ('힣' as u16, (Some('ㅎ' as u16), Some('ㅣ' as u16), Some('ㅎ' as u16))),
        ('밖' as u16, (Some('ㅂ' as u16), Some('ㅏ' as u16), Some('ㄲ' as u16))),
    ];

    for (syl, (cho, joong, jong)) in samples.into_iter() {
        assert!(KorChar::from_u16(syl).unwrap() == KorChar::combine(cho, joong, jong).unwrap());
        assert_eq!(KorChar::combine(cho, joong, jong).unwrap().to_u16(), syl);
        assert!(is_hangul(KorChar::combine(cho, joong, jong).unwrap().to_u16()));
    }

    let invalids = vec![
        ((Some('ㄶ' as u16), Some('ㅏ' as u16), Some('ㅊ' as u16)), KorError::InvalidCho('ㄶ' as u16)),
        ((Some('ㅂ' as u16), Some('ㄲ' as u16), Some('ㄲ' as u16)), KorError::InvalidJoong('ㄲ' as u16)),
        ((Some('ㅂ' as u16), Some('ㅏ' as u16), Some('ㄸ' as u16)), KorError::InvalidJong('ㄸ' as u16)),
        ((Some('ㅂ' as u16), Some(65533), Some('ㄱ' as u16)), KorError::InvalidJoong(65533)),
    ];

    for ((cho, joong, jong), err) in invalids.into_iter() {
        println!("{err}, {}", KorChar::combine(cho, joong, jong).unwrap_err());
        assert_eq!(KorChar::combine(cho, joong, jong), Err(err));
    }
}

#[test]
fn u16_conversion_test() {
    for c in 12000..60000 {
        match KorChar::from_u16(c) {
            Ok(k) => {
                assert!(is_hangul(c));
                assert_eq!(c, k.to_u16());
            },
            _ => {
                assert!(!is_hangul(c));
            }
        }
    }
}

#[test]
fn consonant_assembly_test() {
    let compounds = vec![
        'ㄲ' as u16, 'ㄳ' as u16, 'ㄵ' as u16, 'ㄶ' as u16, 'ㄸ' as u16,
        'ㄺ' as u16, 'ㄻ' as u16, 'ㄼ' as u16, 'ㄽ' as u16, 'ㄾ' as u16,
        'ㄿ' as u16, 'ㅀ' as u16, 'ㅃ' as u16, 'ㅄ' as u16, 'ㅆ' as u16,
        'ㅉ' as u16, 'ㅘ' as u16, 'ㅙ' as u16, 'ㅚ' as u16, 'ㅝ' as u16,
        'ㅞ' as u16, 'ㅟ' as u16, 'ㅢ' as u16,
    ];

    for c in 12000..60000 {
        match disassemble(c) {
            Some((c1, c2)) => {
                assert!(compounds.contains(&c));
                assert_eq!(assemble(c1, c2).unwrap(), c);
            }
            None => {
                assert!(!compounds.contains(&c));
            }
        }
    }
}

#[test]
fn korean_qwerty_conversion_test() {
    let samples = vec![
        ("", ""),
        ("ㄹ", "f"),
        ("리", "fl"),
        ("린", "fls"),
        ("리누", "flsn"),
        ("리눅", "flsnr"),
        ("리눇", "flsnrt"),
        ("리눅스", "flsnrtm"),
        ("예비군 귀찮아", "dPqlrns rnlcksgdk"),
        ("가까나다따라마바빠사싸", "rkRkskekEkfkakqkQktkTk"),
        ("가갸개걔", "rkrirorO"),
        ("그녀의 친구라도 이 노랠 듣는다면 그녀에게 전해줘요 내가 아직 사랑한다고~", "rmsudml clsrnfkeh dl shfof emesmsekaus rmsudprp wjsgownjdy sork dkwlr tkfkdgksekrh~"),
        ("우리가 처음 만난 그 시간 그 자리에 내가 매일 기다린다고~", "dnflrk cjdma akssks rm tlrks rm wkfldp sork aodlf rlekflsekrh~"),
        ("값비싼 손목시계", "rkqtqlTks thsahrtlrP"),
        ("앉아서 이빨 뽑아", "dkswdktj dlQkf Qhqdk"),
        ("내 몫 챙겨", "so ahrt codru"),
        ("밖에서", "qkRdptj"),
        ("ㄱㄱ", "rr"),
        ("우끼욱기", "dnRldnrrl"),
        ("내일 바빠?", "sodlf qkQk?"),
        ("바ㅃ", "qkQ"),
        ("ㅣ", "l"),
        ("ㅢ", "ml"),
        ("ㅢ ㅢ", "ml ml"),
        ("ㄳ", "rt"),
        ("ㄱ시", "rtl"),
        ("ㄱ싱", "rtld"),
        ("ㄱ식시", "rtlrtl"), 
        ("ㄱ시띠", "rtlEl"),
        ("ㄱ시ㄸ", "rtlE"),
        ("곽", "rhkr"),
    ];

    for (korean, qwerty) in samples.into_iter() {
        let korean = korean.encode_utf16().collect::<Vec<u16>>();
        let qwerty = qwerty.encode_utf16().collect::<Vec<u16>>();

        assert_eq!(qwerty_to_kor(&qwerty), korean);
        assert_eq!(kor_to_qwerty(&korean), qwerty);
        assert_eq!(qwerty_to_kor(&kor_to_qwerty(&qwerty_to_kor(&qwerty))), korean);
        assert_eq!(kor_to_qwerty(&qwerty_to_kor(&kor_to_qwerty(&korean))), qwerty);

        for c in korean.into_iter() {
            if is_hangul(c) {
                let new_kor_char = KorChar::from_u16(c).unwrap();
                let new_qwerty = new_kor_char.to_qwerty();
                let new_new_kor_char = KorChar::from_qwerty(&new_qwerty).unwrap();

                assert_eq!(c, new_new_kor_char.to_u16());
            }
        }
    }
}

#[test]
fn validity_checkers() {
    let chos = CHOS.to_vec();
    let joongs = JOONGS.to_vec();
    let jongs = JONGS.to_vec();

    for c in 12000..13000 {
        if chos.contains(&c) {
            assert!(is_valid_cho(c));
            assert!(is_hangul(c));
        }

        else {
            assert!(!is_valid_cho(c));
        }

        if joongs.contains(&c) {
            assert!(is_valid_joong(c));
            assert!(is_hangul(c));
        }

        else {
            assert!(!is_valid_joong(c));
        }

        if jongs.contains(&c) {
            assert!(is_valid_jong(c));
            assert!(is_hangul(c));
        }

        else {
            assert!(!is_valid_jong(c));
        }
    }
}
