/*
 * chamkho - a word breaker written in Rust
 * Copyright (C) 2015  Vee Satayamas
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

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
