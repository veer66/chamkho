use dict::Dict;
use graph_builder::GraphBuilder;
use edge::{Edge};

#[derive(Debug, PartialEq)]
pub struct TextRange {
    pub s: usize,
    pub e: usize
}

pub struct Graph {
    edges: Vec<Edge>,
    txt: Vec<char>
}
    
impl Graph {
    pub fn build(_txt: &str, dict: &Dict) -> Graph {
        let txt: Vec<char> = _txt.chars().collect();
        let mut g = Vec::with_capacity(txt.len() + 1);
        {
            let mut builder = GraphBuilder::new(&txt, &mut g, dict);
            builder.build();
        }
        Graph{edges:g, txt:txt}
    }

    pub fn to_ranges(&self) -> Vec<TextRange> {
        let len = self.edges.len();
        let mut ranges: Vec<TextRange> = Vec::with_capacity(len);
        let mut e = len-1;
        while e > 0 {

            let edge = self.edges[e];
            let s = edge.p;
            ranges.push(TextRange{s:s, e:e});
            e = s;            
        }
        ranges.reverse();
        return ranges
    }

    pub fn to_str_vec(&self) -> Vec<String> {
        let ranges = self.to_ranges();
        let mut str_vec: Vec<String> = Vec::with_capacity(ranges.len());
        for r in ranges {
            let mut buf = String::with_capacity(3 * (r.e - r.s + 1));
            for i in r.s..r.e {
                buf.push(self.txt[i]);
            }
            str_vec.push(buf)
        }
        str_vec
    }
}