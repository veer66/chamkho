pub extern crate wordcut_engine;

use self::wordcut_engine::Dict;
use std::io::Result;
use std::path::Path;

pub type Wordcut = self::wordcut_engine::Wordcut;

macro_rules! insert_prefix {
    ($filename:expr) => {
        if cfg!(feature = "onedir") {
            Path::new(concat!("./", $filename))
        } else {
            Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/data/", $filename))
        }
    };
}

macro_rules! insert_prefix_str {
    ($filename:expr) => {
        if cfg!(feature = "onedir") {
            concat!("./", $filename)
        } else {
            concat!(env!("CARGO_MANIFEST_DIR"), "/data/", $filename)
        }
    };
}
  
pub fn default_path() -> &'static Path {
    insert_prefix!("words_th.txt")
}

pub fn lao_path() -> &'static Path {
    insert_prefix!("laowords.txt")
}

pub fn khmer_dict_path() -> &'static Path {
    insert_prefix!("khmerdict.txt")
}

pub fn myanmar_dict_path() -> &'static Path {
    insert_prefix!("myanmar-dict.txt")
}

pub fn thai_cluster_path() -> Option<String> {
    Some(insert_prefix_str!("thai_cluster_rules.txt").to_owned())
}

pub fn lao_clusters_path() -> Option<String> {
    None
}

pub fn khmer_clusters_path() -> Option<String> {
    Some(insert_prefix_str!("khmer_cluster_rules.txt").to_owned())
}

pub fn myanmar_clusters_path() -> Option<String> {
    None
}

pub fn load_dict(path: &Path) -> Result<Dict> {
    self::wordcut_engine::load_dict(path)
}

pub fn thai_replace_rules_path() -> Option<String> {
    Some(insert_prefix_str!("thai-replace-rules.json").to_owned())
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
}
