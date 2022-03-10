//! Provides functions for converting between the different character sets in the Japanese language.

use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;

use crate::charset::*;
use crate::vowel::*;

struct TwoWayMap {
    normal: HashMap<Vowel, char>,
    reversed: HashMap<char, Vowel>,
}

impl TwoWayMap {
    pub fn new(normal: HashMap<Vowel, char>) -> TwoWayMap {
        let reversed: HashMap<_, _> = normal.iter().map(|(k, v)| (*v, *k)).collect();

        TwoWayMap { normal, reversed }
    }
}

const HIRAGANA_KATAKANA_DIFF: u32 = 'ア' as u32 - 'あ' as u32;

lazy_static! {
    static ref VOWEL_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'あ',
        Vowel::I => 'い',
        Vowel::U => 'う',
        Vowel::E => 'え',
        Vowel::O => 'お',
    });
    static ref VOWEL_SMALL_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'ぁ',
        Vowel::I => 'ぃ',
        Vowel::U => 'ぅ',
        Vowel::E => 'ぇ',
        Vowel::O => 'ぉ',
    });
    static ref K_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'か',
        Vowel::I => 'き',
        Vowel::U => 'く',
        Vowel::E => 'け',
        Vowel::O => 'こ',
    });
    static ref G_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'が',
        Vowel::I => 'ぎ',
        Vowel::U => 'ぐ',
        Vowel::E => 'げ',
        Vowel::O => 'ご',
    });
    static ref S_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'さ',
        Vowel::I => 'し',
        Vowel::U => 'す',
        Vowel::E => 'せ',
        Vowel::O => 'そ',
    });
    static ref Z_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'ざ',
        Vowel::I => 'じ',
        Vowel::U => 'ず',
        Vowel::E => 'ぜ',
        Vowel::O => 'ぞ',
    });
    static ref T_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'た',
        Vowel::I => 'ち',
        Vowel::U => 'つ',
        Vowel::E => 'て',
        Vowel::O => 'と',
    });
    static ref D_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'だ',
        Vowel::I => 'ぢ',
        Vowel::U => 'づ',
        Vowel::E => 'で',
        Vowel::O => 'ど',
    });
    static ref N_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'な',
        Vowel::I => 'に',
        Vowel::U => 'ぬ',
        Vowel::E => 'ね',
        Vowel::O => 'の',
    });
    static ref H_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'は',
        Vowel::I => 'ひ',
        Vowel::U => 'ふ',
        Vowel::E => 'へ',
        Vowel::O => 'ほ',
    });
    static ref B_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'ば',
        Vowel::I => 'び',
        Vowel::U => 'ぶ',
        Vowel::E => 'べ',
        Vowel::O => 'ぼ',
    });
    static ref P_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'ぱ',
        Vowel::I => 'ぴ',
        Vowel::U => 'ぷ',
        Vowel::E => 'ぺ',
        Vowel::O => 'ぽ',
    });
    static ref M_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'ま',
        Vowel::I => 'み',
        Vowel::U => 'む',
        Vowel::E => 'め',
        Vowel::O => 'も',
    });
    static ref R_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'ら',
        Vowel::I => 'り',
        Vowel::U => 'る',
        Vowel::E => 'れ',
        Vowel::O => 'ろ',
    });
    static ref Y_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'や',
        Vowel::U => 'ゆ',
        Vowel::O => 'よ',
    });
    static ref Y_SMALL_MAP: TwoWayMap = TwoWayMap::new(hashmap! {
        Vowel::A => 'ゃ',
        Vowel::U => 'ゅ',
        Vowel::O => 'ょ',
    });
    static ref MAPS: Vec<&'static TwoWayMap> = vec![
        &VOWEL_MAP,
        &VOWEL_SMALL_MAP,
        &K_MAP,
        &G_MAP,
        &S_MAP,
        &Z_MAP,
        &T_MAP,
        &D_MAP,
        &N_MAP,
        &H_MAP,
        &B_MAP,
        &P_MAP,
        &M_MAP,
        &R_MAP,
        &Y_MAP,
        &Y_SMALL_MAP,
    ];
}

fn get_map_for_hiragana(hiragana: char) -> Option<&'static TwoWayMap> {
    for map in MAPS.iter() {
        if map.reversed.contains_key(&hiragana) {
            return Some(map);
        }
    }

    None
}

pub fn get_vowel_for_hiragana(hiragana: char) -> Vowel {
    let map = get_map_for_hiragana(hiragana);
    match map {
        Some(v) => *v.reversed.get(&hiragana).unwrap(),
        None => panic!("Couldn't find map for hiragana '{hiragana}'."),
    }
}

pub fn convert_katakana_to_hiragana(katakana: char) -> char {
    char::from_u32(katakana as u32 - HIRAGANA_KATAKANA_DIFF).expect("Expected valid char.")
}

pub fn convert_hiragana_to_katakana(hiragana: char) -> char {
    char::from_u32(hiragana as u32 + HIRAGANA_KATAKANA_DIFF).expect("Expected valid char.")
}

pub fn convert_vowel_in_stem(hiragana: char, to_vowel: Vowel) -> char {
    if !is_hiragana(hiragana) {
        return hiragana;
    }

    let is_special_wa = hiragana == 'わ';

    // Special handling for わ in stems.
    let map_option = if is_special_wa {
        Some(&*VOWEL_MAP)
    } else {
        get_map_for_hiragana(hiragana)
    };
    let map = match map_option {
        Some(v) => v,
        None => return hiragana,
    };

    // If the chosen map is the vowel map and we want to change it to A...
    if std::ptr::eq(map, &*VOWEL_MAP) && to_vowel == Vowel::A {
        return 'わ';
    }

    *map.normal.get(&to_vowel).unwrap()
}

pub fn get_prolonged_hiragana_for_vowel(vowel: Vowel) -> char {
    match vowel {
        Vowel::A => 'あ',
        Vowel::I => 'い',
        Vowel::U => 'う',
        Vowel::E => 'い',
        Vowel::O => 'う',
    }
}

pub fn convert_katakana_to_hiragana_string(katakana: &str) -> String {
    let mut hiragana_string = String::with_capacity(katakana.len());

    let chars: Vec<_> = katakana.chars().collect();
    for (i, ch) in chars.iter().copied().enumerate() {
        let hiragana_ch: char;

        if is_hiragana(ch) || !is_katakana(ch) {
            hiragana_ch = ch;
        } else if ch == 'ー' {
            let previous_ch = chars[i - 1];
            let vowel = get_vowel_for_hiragana(convert_katakana_to_hiragana(previous_ch));
            hiragana_ch = get_prolonged_hiragana_for_vowel(vowel);
        } else {
            hiragana_ch = convert_katakana_to_hiragana(ch);
        }
        hiragana_string.push(hiragana_ch);
    }

    hiragana_string
}

pub fn convert_hiragana_to_katakana_string(hiragana: &str) -> String {
    let mut katakana_string = String::with_capacity(hiragana.len());

    let chars: Vec<_> = hiragana.chars().collect();
    for ch in chars.iter().copied() {
        let katakana_ch = if is_katakana(ch) || !is_hiragana(ch) {
            ch
        } else {
            convert_hiragana_to_katakana(ch)
        };
        katakana_string.push(katakana_ch);
    }

    katakana_string
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case('わ', Vowel::I, 'い')]
    #[case('い', Vowel::A, 'わ')]
    #[case('き', Vowel::A, 'か')]
    #[case('し', Vowel::E, 'せ')]
    fn convert_vowel_in_stem_test(
        #[case] hiragana: char,
        #[case] to_vowel: Vowel,
        #[case] expected: char,
    ) {
        assert_eq!(expected, convert_vowel_in_stem(hiragana, to_vowel));
    }

    #[rstest]
    #[case("モン", "もん")]
    #[case("キヨウビ", "きようび")]
    #[case("キョウビ", "きょうび")]
    #[case("キヨービ", "きようび")]
    #[case("キョービ", "きょうび")]
    #[case("キープ", "きいぷ")]
    fn convert_katakana_to_hiragana_test(#[case] katakana: &str, #[case] expected: &str) {
        assert_eq!(expected, convert_katakana_to_hiragana_string(katakana));
    }

    #[rstest]
    #[case("もん", "モン")]
    #[case("ひトリ", "ヒトリ")]
    #[case("きようび", "キヨウビ")]
    fn convert_hiragana_to_katakana_test(#[case] katakana: &str, #[case] expected: &str) {
        assert_eq!(expected, convert_hiragana_to_katakana_string(katakana));
    }
}
