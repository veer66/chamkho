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

}
