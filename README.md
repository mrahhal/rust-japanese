# rust-japanese

[![CI](https://github.com/mrahhal/rust-japanese/actions/workflows/ci.yml/badge.svg)](https://github.com/mrahhal/rust-japanese/actions/workflows/ci.yml)
[![Crate][crate-image]][crate-link]
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Recognize the different Japanese scripts and convert between hiragana/katakana.

## Install

Not published yet.

<!-- Add this to your `Cargo.toml`:

```toml
[dependencies]
japanese = "{version}"
```

Visit the [crate-link](crate) page to copy the latest version. -->

## Documentation

https://docs.rs/japanese

## Usage

There are two primary modules you'll be using from this crate:

- charset: Contains functions that detect different scripts (hiragana, katakana, kanji, jp punctuation, etc).
- converter: Contains functions that convert between different scripts, in addition to other useful utilities.

### charset

```rs
use japanese::charset;

charset::is_japanese('あ') // -> true
charset::is_japanese_punctuation('。') // -> true
charset::is_katakana_string("あまり") // -> true
// ...
```

### converter

```rs
use japanese::converter;

converter::convert_hiragana_to_katakana_string("もん") // -> モン
converter::convert_katakana_to_hiragana_string("キョービ") // -> きょうび
```

[//]: # "links"
[crate-image]: https://img.shields.io/crates/v/japanese.svg
[crate-link]: https://crates.io/crates/japanese
