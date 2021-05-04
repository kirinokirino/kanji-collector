#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
mod utils;

use serde::{Deserialize, Serialize};
use std::{str::Chars, vec::Vec};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct KanjiList {
    pub kanjis: Vec<String>,
}

impl From<Vec<String>> for KanjiList {
    fn from(vec: Vec<String>) -> Self {
        Self { kanjis: vec }
    }
}

#[wasm_bindgen]
pub fn search_kanji(line: &str) -> JsValue {
    let chars = line.trim().chars();
    let res = collect_kanjis(chars);
    JsValue::from_serde(&res).unwrap()
}

pub fn collect_kanjis(chars: Chars) -> KanjiList {
    let mut list = KanjiList { kanjis: vec![] };
    let mut extension = "".to_string();
    for ch in chars {
        if is_char_between_char_range(ch, KANJI_BEG, KANJI_END) {
            extension.push(ch);
        } else {
            if !extension.is_empty() {
                list.kanjis.push(extension);
            }
            extension = "".to_string();
        }
    }
    list
}

// https://github.com/kitallis/konj
// taken from this project's `strings` file.
pub fn is_char_between_char_range(ch: char, range_beg: char, range_end: char) -> bool {
    if !(ch >= range_beg && ch <= range_end) {
        // || ch.is_whitespace()) {
        return false;
    }

    true
}

// https://github.com/kitallis/konj
// taken from this project's `constants` file.
pub const HIRAGANA_BEG: char = '\u{3040}';
pub const HIRAGANA_END: char = '\u{309F}';
pub const KATAKANA_BEG: char = '\u{30A0}';
pub const KATAKANA_END: char = '\u{30FF}';
pub const KANJI_BEG: char = '\u{4E00}';
pub const KANJI_END: char = '\u{9FAF}';
