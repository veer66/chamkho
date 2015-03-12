use dict::{Dict,Policy};

#[derive(Debug)]
pub struct DictAcc {
    l: usize,
    r: usize,
    pub offset: usize,
    pub is_final: bool
}

impl DictAcc {
    pub fn new(l: usize, r: usize) -> DictAcc {
        DictAcc{l:l, r:r, offset:0, is_final: false}
    }

    pub fn transit(&self, ch: char, d: &Dict) -> Option<DictAcc> {
        match d.seek(Policy::Left, self.l, self.r, self.offset, ch) {
            Some(l) => {
                match d.seek(Policy::Right, l, self.r, self.offset, ch) {
                    Some(r) => {
                        Some(DictAcc{l:l, r:r, offset:self.offset+1,
                                     is_final: d.is_final(self.offset+1, l)})
                    },
                    None => None
                }
            },
            None => None
        }
    }
}