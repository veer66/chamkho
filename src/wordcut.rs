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
}




/*
fn main() {
//trim_right_matches('\n')
    let _dict = Dict::load("tdict-std.txt").unwrap();
    let mut reader = io::stdin();
    let mut o = io::stdout();
    loop {
        let input = match reader.read_line() {
            Ok(line) => line,
            Err(_) => break
        };        

        let segmented: &str = &(g.to_str_vec().connect("|"))[..];
        o.write_str(segmented);
        o.write_str("\n");
    }
} 
*/
