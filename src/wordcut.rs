use dict::Dict;
use graph::{Graph, TextRange};

pub struct Wordcut {
    dict: Dict
}

impl Wordcut {
    pub fn new(dict: Dict) -> Wordcut {
        Wordcut{dict: dict}
    }
    
    pub fn segment(&self, txt: &String) -> Vec<TextRange> {
        let g = Graph::build(&txt[..], &self.dict);
        g.to_ranges()
    }

    pub fn segment_into_strings(&self, txt: &String) -> Vec<String> {
        let g = Graph::build(&txt[..], &self.dict);
        g.to_str_vec()
    }

    pub fn put_delimiters(&self, txt: &String, delim: &str) -> String {
        self.segment_into_strings(txt).connect(delim)
    }
}