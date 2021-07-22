/*
 * chamkho - a word breaker written in Rust
 * Copyright (C) 2015  Vee Satayamas
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

pub extern crate wordcut_engine;

use self::wordcut_engine::Dict;
use std::io::Result;
use std::path::Path;

pub type Wordcut = self::wordcut_engine::Wordcut;

#[allow(dead_code)]
pub fn cargo_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn default_path() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/data/words_th.txt"))
}

pub fn lao_path() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/data/laowords.txt"))
}

pub fn thai_cluster_path() -> Option<String> {
    Some(concat!(env!("CARGO_MANIFEST_DIR"), "/data/thai_cluster_rules.txt").to_owned())
}

pub fn lao_clusters_path() -> Option<String> {
    None
}

pub fn load_dict(path: &Path) -> Result<Dict> {
    self::wordcut_engine::load_dict(path)
}

#[cfg(test)]
mod tests {
    use super::wordcut_engine::TextRange;
    use super::Wordcut;
    use std::path::Path;

    #[test]
    fn test_default_dict() {
        let dict = super::load_dict(super::default_path());
        assert!(dict.is_ok());
    }

    #[test]
    fn test_segment_into_ranges() {
        let dict = super::load_dict(Path::new("data/thai.txt")).unwrap();
        let wordcut = Wordcut::new(dict);
        let ranges = wordcut.segment("กากกา");
        let expected = vec![TextRange { s: 0, e: 3 }, TextRange { s: 3, e: 5 }];
        assert_eq!(ranges, expected)
    }

    #[test]
    fn test_segment_into_strings() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("กากกา");
        let expected = vec!["กาก".to_string(), "กา".to_string()];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_segment_with_punc() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("\"ฆกากา\"");
        let expected = &vec!["\"", "ฆ", "กา", "กา", "\""]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_segment_with_parentheses() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings(&"(ฆกากา)".to_string());
        let expected = &vec!["(", "ฆ", "กา", "กา", ")"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_segment_with_unicode_quote() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("“ฆกากา”");
        let expected = &vec!["“", "ฆ", "กา", "กา", "”"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_segment_unknown_sandwich() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("ฮฮกาฮฮ");
        let expected = &vec!["ฮฮ", "กา", "ฮฮ"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_segment_2spaces() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("กา  มา");
        let expected = &vec!["กา", "  ", "มา"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_segment_2spaces_unk() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("ฮฮ  ญญ");
        let expected = &vec!["ฮฮ", "  ", "ญญ"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_one_word() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("มาตรา");
        let expected = &vec!["มาตรา"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_from_law() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings("มาตรา 482 ผู้ขายไม่");
        let expected = &vec!["มาตรา", " ", "482", " ", "ผู้", "ขาย", "ไม่"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_space() {
        let dict = super::load_dict(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment("a cat");
        let expected = vec![
            TextRange { s: 0, e: 1 },
            TextRange { s: 1, e: 2 },
            TextRange { s: 2, e: 5 },
        ];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_lao() {
        let dict = super::load_dict(super::lao_path());
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment(&"ພາສາລາວມີ".to_string());
        let expected = vec![
            TextRange { s: 0, e: 4 },
            TextRange { s: 4, e: 7 },
            TextRange { s: 7, e: 9 },
        ];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_latin() {
        let dict = super::load_dict(super::lao_path());
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings(&"ฑฑACญญ".to_string());
        let expected = &vec!["ฑฑ", "AC", "ญญ"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_get_cargo_dir() {
        let cargo_dir = super::cargo_dir().to_str().unwrap();
        assert!(cargo_dir.len() > 0);
    }
}
