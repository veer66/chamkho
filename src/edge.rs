#[derive(PartialEq, Eq, Copy, Debug)]
pub enum EdgeType { INIT, DICT, UNK }

#[derive(Copy, Debug)]
pub struct Edge {
    pub w: usize,
    pub unk: usize,
    pub p: usize,
    pub etype: EdgeType
}

impl Edge {
    pub fn better_than(&self, o: &Edge) -> bool {
        (self.unk < o.unk) || (self.unk == o.unk && self.w < o.w)
    }
}
