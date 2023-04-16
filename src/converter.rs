//! Provides functions for converting between the different character sets in the Japanese language.
//!
//! ### Leniency
//!
//! Conversion functions assume a lenient approach in which the same `char` is returned if conversion can't be done.
//!
//! As an example, `convert_katakana_to_hiragana` expects a katakana character. If a non-katakana `char` is provided then the same `char` will be returned.
//!
//! In conversion functions that work with strings, any offending chars are simply skipped over (i.e included in the output string as is).

use maplit::hashmap;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::{charset, Vowel};

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

static VOWEL_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'あ',
        Vowel::I => 'い',
        Vowel::U => 'う',
        Vowel::E => 'え',
        Vowel::O => 'お',
    })
});
static VOWEL_SMALL_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'ぁ',
        Vowel::I => 'ぃ',
        Vowel::U => 'ぅ',
        Vowel::E => 'ぇ',
        Vowel::O => 'ぉ',
    })
});
static K_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'か',
        Vowel::I => 'き',
        Vowel::U => 'く',
        Vowel::E => 'け',
        Vowel::O => 'こ',
    })
});
static G_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'が',
        Vowel::I => 'ぎ',
        Vowel::U => 'ぐ',
        Vowel::E => 'げ',
        Vowel::O => 'ご',
    })
});
static S_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'さ',
        Vowel::I => 'し',
        Vowel::U => 'す',
        Vowel::E => 'せ',
        Vowel::O => 'そ',
    })
});
static Z_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'ざ',
        Vowel::I => 'じ',
        Vowel::U => 'ず',
        Vowel::E => 'ぜ',
        Vowel::O => 'ぞ',
    })
});
static T_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'た',
        Vowel::I => 'ち',
        Vowel::U => 'つ',
        Vowel::E => 'て',
        Vowel::O => 'と',
    })
});
static D_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'だ',
        Vowel::I => 'ぢ',
        Vowel::U => 'づ',
        Vowel::E => 'で',
        Vowel::O => 'ど',
    })
});
static N_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'な',
        Vowel::I => 'に',
        Vowel::U => 'ぬ',
        Vowel::E => 'ね',
        Vowel::O => 'の',
    })
});
static H_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'は',
        Vowel::I => 'ひ',
        Vowel::U => 'ふ',
        Vowel::E => 'へ',
        Vowel::O => 'ほ',
    })
});
static B_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'ば',
        Vowel::I => 'び',
        Vowel::U => 'ぶ',
        Vowel::E => 'べ',
        Vowel::O => 'ぼ',
    })
});
static P_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'ぱ',
        Vowel::I => 'ぴ',
        Vowel::U => 'ぷ',
        Vowel::E => 'ぺ',
        Vowel::O => 'ぽ',
    })
});
static M_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'ま',
        Vowel::I => 'み',
        Vowel::U => 'む',
        Vowel::E => 'め',
        Vowel::O => 'も',
    })
});
static R_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'ら',
        Vowel::I => 'り',
        Vowel::U => 'る',
        Vowel::E => 'れ',
        Vowel::O => 'ろ',
    })
});
static Y_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'や',
        Vowel::U => 'ゆ',
        Vowel::O => 'よ',
    })
});
static Y_SMALL_MAP: Lazy<TwoWayMap> = Lazy::new(|| {
    TwoWayMap::new(hashmap! {
        Vowel::A => 'ゃ',
        Vowel::U => 'ゅ',
        Vowel::O => 'ょ',
    })
});
static MAPS: Lazy<Vec<&'static TwoWayMap>> = Lazy::new(|| {
    vec![
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
    ]
});

fn get_map_for_hiragana(hiragana: char) -> Option<&'static TwoWayMap> {
    for map in MAPS.iter() {
        if map.reversed.contains_key(&hiragana) {
            return Some(map);
        }
    }

    None
}

/// Gets the `Vowel` of the given hiragana `char`.
pub fn get_vowel_for_hiragana(hiragana: char) -> Option<Vowel> {
    let map = get_map_for_hiragana(hiragana)?;
    map.reversed.get(&hiragana).copied()
}

/// Converts a hiragana `char` to another [Vowel] according to how agglutination works in stems.
///
/// This basically means we have to add special handling of わ.
///
/// One example is when the char is a vowel itself and we want to convert it to `Vowel::A`.
/// In this case わ will be returned.
///
/// In case there's anything wrong in the process, the same provided `char` will be returned.
pub fn convert_to_vowel_in_stem(hiragana: char, to_vowel: Vowel) -> char {
    if !charset::is_hiragana(hiragana) {
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

    // If the chosen map is the vowel map and we want to change it to A we'll just return わ.
    if std::ptr::eq(map, &*VOWEL_MAP) && to_vowel == Vowel::A {
        return 'わ';
    }

    *map.normal.get(&to_vowel).unwrap()
}

/// Converts the given katakana `char` to hiragana.
pub fn convert_katakana_to_hiragana(katakana: char) -> char {
    if !charset::is_katakana(katakana) {
        return katakana;
    }
    char::from_u32(katakana as u32 - HIRAGANA_KATAKANA_DIFF).unwrap_or(katakana)
}

/// Converts the given hiragana `char` to katakana.
pub fn convert_hiragana_to_katakana(hiragana: char) -> char {
    if !charset::is_hiragana(hiragana) {
        return hiragana;
    }
    char::from_u32(hiragana as u32 + HIRAGANA_KATAKANA_DIFF).unwrap_or(hiragana)
}

/// Gets the prolonged hiragana `char` that's used when the preceding character has the given `Vowel`.
fn get_prolonged_hiragana_for_vowel(vowel: Vowel) -> char {
    match vowel {
        Vowel::A => 'あ',
        Vowel::I => 'い',
        Vowel::U => 'う',
        Vowel::E => 'い',
        Vowel::O => 'う',
    }
}

/// Converts the given katakana string to hiragana.
///
/// This takes care of 'ー' used in prolonged voices. i.e キョービ -> きょうび
pub fn convert_katakana_to_hiragana_string(katakana: &str) -> String {
    let mut hiragana_string = String::with_capacity(katakana.len());

    let chars: Vec<_> = katakana.chars().collect();
    for (i, ch) in chars.iter().copied().enumerate() {
        let hiragana_ch = if charset::is_hiragana(ch) || !charset::is_katakana(ch) {
            ch
        } else if ch == 'ー' {
            if i == 0 {
                // Nothing before it, no need to convert.
                ch
            } else {
                let previous_ch = chars[i - 1];

                // There's special handling for ワ and ヲ.
                match previous_ch {
                    'ワ' => 'あ',
                    'ヮ' => 'ぁ',
                    'ヲ' => 'お',
                    _ => {
                        // Standard case.
                        let vowel = get_vowel_for_hiragana(convert_katakana_to_hiragana(previous_ch));
                        if let Some(vowel) = vowel {
                            // A valid converted hiragana that we the vowel of.
                            get_prolonged_hiragana_for_vowel(vowel)
                        } else {
                            // We don't know how to do this, return the same character.
                            ch
                        }
                    }
                }
            }
        } else {
            convert_katakana_to_hiragana(ch)
        };
        hiragana_string.push(hiragana_ch);
    }

    hiragana_string
}

/// Converts the given hiragana string to katakana.
///
/// This **does not** change long voices into 'ー'. i.e きょうび -> キョウビ
pub fn convert_hiragana_to_katakana_string(hiragana: &str) -> String {
    let mut katakana_string = String::with_capacity(hiragana.len());

    let chars: Vec<_> = hiragana.chars().collect();
    for ch in chars.iter().copied() {
        let katakana_ch = if charset::is_katakana(ch) || !charset::is_hiragana(ch) {
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
        assert_eq!(expected, convert_to_vowel_in_stem(hiragana, to_vowel));
    }

    #[rstest]
    fn convert_katakana_to_hiragana_returns_same_char_if_invalid() {
        assert_eq!('a', convert_katakana_to_hiragana('a'));
    }

    #[rstest]
    fn convert_hiragana_to_katakana_returns_same_char_if_invalid() {
        assert_eq!('a', convert_hiragana_to_katakana('a'));
    }

    #[rstest]
    #[case("モン", "もん")]
    #[case("キヨウビ", "きようび")]
    #[case("キョウビ", "きょうび")]
    #[case("キヨービ", "きようび")]
    #[case("キョービ", "きょうび")]
    #[case("キープ", "きいぷ")]
    #[case("チームワーク", "ちいむわあく")]
    #[case("ヲー", "をお")]
    #[case("ー", "ー")]
    fn convert_katakana_to_hiragana_string_test(#[case] katakana: &str, #[case] expected: &str) {
        assert_eq!(expected, convert_katakana_to_hiragana_string(katakana));
    }

    #[rstest]
    #[case("チームワーク")]
    #[case("ヲー")]
    #[case("ー")]
    fn convert_katakana_to_hiragana_string_does_not_panic(#[case] katakana: &str) {
        convert_katakana_to_hiragana_string(katakana);
    }

    #[rstest]
    #[case("もん", "モン")]
    #[case("ひトリ", "ヒトリ")]
    #[case("きようび", "キヨウビ")]
    fn convert_hiragana_to_katakana_string_test(#[case] katakana: &str, #[case] expected: &str) {
        assert_eq!(expected, convert_hiragana_to_katakana_string(katakana));
    }
}
