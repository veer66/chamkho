use edge::{Edge,EdgeType};
use acc::DictAcc;
use dict::Dict;

pub struct GraphBuilder<'a> {
    dict_acc: Vec<DictAcc>,
    dict: &'a Dict,
    pub g: &'a mut Vec<Edge>,
    pub txt: &'a Vec<char>
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

impl<'a> GraphBuilder<'a> {
    pub fn new(txt: &'a Vec<char>, g: &'a mut  Vec<Edge>, dict: &'a Dict) -> GraphBuilder<'a> {
        g.push(Edge{w:0, unk:0, p: 0, etype: EdgeType::INIT});
        GraphBuilder{dict_acc:vec![],
                     dict:dict,
                     g:g,
                     txt:txt}
    }

    pub fn transit(&mut self, ch: char) {
        self.transit_dict(ch);
    }

    fn transit_dict(&mut self, ch: char) {
        self.dict_acc.push(DictAcc::new(0, self.dict.r()));
        self.dict_acc = self.dict_acc
            .iter()
            .map(|a| a.transit(ch, &self.dict))
            .filter(|t| t.is_some())
            .map(|t| t.unwrap())
            .collect()
    }

    fn build_edges(&mut self, i: usize, left: usize) -> Vec<Edge> {
        let edges: Vec<Edge> = self.dict_acc
            .iter()
            .filter(|a| a.is_final)
            .map(|a| {
                let p = 1 + i - a.offset;
                let src = &self.g[p];
                Edge{w:src.w+1, unk:src.unk, p:p, etype:EdgeType::DICT}
            }).collect();
        if edges.len() > 0 {
            edges
        } else {
            let src = &self.g[left];
            vec![Edge{w:src.w+1, unk:src.unk, p:left, etype:EdgeType::UNK}]
        }
    }

    pub fn build(&mut self) {
        let mut left = 0;
        for i in 0..self.txt.len() {
            let ch = self.txt[i];
            self.transit(ch);
            let edges = self.build_edges(i, left);
            let e = best_edge(&edges).unwrap();
            let g = &mut self.g;
            let _e = *e.clone();
            g.push(_e);
            if e.etype != EdgeType::UNK {
                left = i + 1;
            }                
        }
    }
}
