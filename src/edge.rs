#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum EdgeType {
    Init,
    Dict,
    Unk,
    InSpace,
    Space
}

#[derive(Clone, Copy, Debug)]
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
