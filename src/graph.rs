use acc::DictAcc;
use dict::Dict;

#[derive(PartialEq, Eq, Copy, Debug)]
enum EdgeType { INIT, DICT, UNK }

#[derive(Copy, Debug)]
struct Edge {
    w: usize,
    unk: usize,
    p: usize,
    etype: EdgeType
}

impl Edge {
    fn better_than(&self, o: &Edge) -> bool {
        (self.unk < o.unk) || (self.unk == o.unk && self.w < o.w)
    }
}

#[derive(Debug, PartialEq)]
pub struct TextRange {
    pub s: usize,
    pub e: usize
}

pub struct Graph {
    edges: Vec<Edge>,
    txt: Vec<char>          
}

fn best_edge(edges: &Vec<Edge>) -> Option<&Edge> {
    if edges.len() > 0 {
        let mut best = &edges[0];
        for i in 1..edges.len() {
            if !best.better_than(&edges[i]) {
                best = &edges[i];
            }
        }
        Some(best)
    } else {
        None
    }
}
    
fn transit(acc: &mut Vec<DictAcc>, ch: char, d: &Dict) -> Vec<DictAcc> {
    acc.push(DictAcc::new(0, d.r()));
    acc
        .iter()
        .map(|a| a.transit(ch, d))
        .filter(|t| t.is_some())
        .map(|t| t.unwrap())
        .collect()
}

fn build_edges(i: usize, acc: &Vec<DictAcc>,
               g: &Vec<Edge>, left: usize) -> Vec<Edge> {
    let edges: Vec<Edge> = acc
        .iter()
        .filter(|a| a.is_final)
        .map(|a| {
            let p = 1 + i - a.offset;
            let src = &g[p];
            Edge{w:src.w+1, unk:src.unk, p:p, etype:EdgeType::DICT}
        }).collect();
    if edges.len() > 0 {
        edges
    } else {
        let src = &g[left];
        vec![Edge{w:src.w+1, unk:src.unk, p:left, etype:EdgeType::UNK}]
    }
}

impl Graph {
    pub fn build(_txt: &str, d: &Dict) -> Graph {
        let txt: Vec<char> = _txt.chars().collect();
        let len = txt.len();
        let mut g:Vec<Edge> = Vec::with_capacity(len + 1);
        g.push(Edge{w:0, unk:0, p: 0, etype: EdgeType::INIT});
        let mut acc: Vec<DictAcc> = vec![];
        let mut left = 0;
        
        for i in 0..len {
            let ch: char = txt[i];
            acc = transit(&mut acc, ch, d);
            let edges = build_edges(i, &acc, &g, left);
            let e = best_edge(&edges).unwrap();
            g.push(*e.clone());
            if e.etype != EdgeType::UNK {
                left = i + 1;
            }                
        }
        Graph{txt:txt, edges:g}
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