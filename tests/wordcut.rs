extern crate chamkho;

#[cfg(test)]
mod tests {
    use chamkho::dict::Dict;
    use chamkho::wordcut::Wordcut;
    use chamkho::graph::TextRange;
    
    #[test]
    fn test_default_dict() {
        let dict = Dict::load_default();
        assert!(dict.is_ok());
        assert!(dict.unwrap().r() > 10000);
    }
    
    #[test]
    fn test_segment_into_ranges() {
        let dict = Dict::load_default();
        let wordcut = Wordcut::new(dict.unwrap());
        let ranges = wordcut.segment(&"กากกา".to_string());
        let expected = vec![TextRange{s:0,e:3}, TextRange{s:3,e:5}];
        assert_eq!(ranges, expected)
    }

    #[test]
    fn test_segment_into_strings() {
        let dict = Dict::load_default();
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings(&"กากกา".to_string());
        let expected = vec!["กาก".to_string(), "กา".to_string()];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_space() {
        let dict = Dict::load_default();
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment(&"a cat".to_string());
        let expected = vec![TextRange{s:0,e:1},TextRange{s:1,e:2},TextRange{s:2,e:5}];
        assert_eq!(words, expected)
    }
}
