//! Provides functions for recognizing the different character sets in the Japanese language.
//!
//! Unicode reference: http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml

const PUNCTUATION_START: char = '\u{3000}';
const PUNCTUATION_END: char = '\u{303f}';
const HIRAGANA_START: char = '\u{3040}';
const HIRAGANA_END: char = '\u{309f}';
const KATAKANA_START: char = '\u{30a0}';
const KATAKANA_END: char = '\u{30ff}';
const FULL_WIDTH_ROMAN_HALF_WIDTH_KATAKANA_START: char = '\u{ff00}';
const FULL_WIDTH_ROMAN_HALF_WIDTH_KATAKANA_END: char = '\u{ffef}';
const KANJI_START: char = '\u{4e00}';
const KANJI_END: char = '\u{9faf}';

pub fn is_japanese(ch: char) -> bool {
    is_hiragana(ch)
        || is_kanji(ch)
        || is_katakana(ch)
        || is_punctuation(ch)
        || is_full_width_roman_half_width_katakana(ch)
}

pub fn is_japanese_character(ch: char) -> bool {
    is_hiragana(ch) || is_kanji(ch) || is_katakana(ch) || is_special_character(ch)
}

/// Detects if the char is a Japanese punctuation (3000 - 303f).
pub fn is_punctuation(ch: char) -> bool {
    ch >= PUNCTUATION_START && ch <= PUNCTUATION_END
}

pub fn is_special_character(ch: char) -> bool {
    ch == '々' || ch == '〆'
}

pub fn is_hiragana(ch: char) -> bool {
    ch >= HIRAGANA_START && ch <= HIRAGANA_END
}

pub fn is_hiragana_string(str: &str) -> bool {
    for ch in str.chars() {
        if !is_hiragana(ch) {
            return false;
        }
    }
    true
}

pub fn is_katakana(ch: char) -> bool {
    ch >= KATAKANA_START && ch <= KATAKANA_END && ch != '・'
}

pub fn is_katakana_string(str: &str) -> bool {
    for ch in str.chars() {
        if !is_katakana(ch) {
            return false;
        }
    }
    true
}

pub fn is_full_width_roman_half_width_katakana(ch: char) -> bool {
    ch >= FULL_WIDTH_ROMAN_HALF_WIDTH_KATAKANA_START
        && ch <= FULL_WIDTH_ROMAN_HALF_WIDTH_KATAKANA_END
}

pub fn is_kana(ch: char) -> bool {
    is_hiragana(ch) || is_katakana(ch)
}

pub fn is_kanji(ch: char) -> bool {
    ch >= KANJI_START && ch <= KANJI_END
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_hiragana_test() {
        assert_eq!(true, is_hiragana_string("きょうだ"));
        assert_eq!(false, is_hiragana_string("勉強ダ"));
    }
}
