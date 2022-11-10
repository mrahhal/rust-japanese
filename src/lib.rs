//! Recognize the different Japanese scripts and convert between hiragana/katakana.

pub mod charset;
pub mod converter;
mod vowel;

pub use vowel::*;
