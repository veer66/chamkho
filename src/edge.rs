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

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum EdgeType {
    Init,
    Dict,
    Unk,
    Punc,
    Latin
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub w: usize,
    pub unk: usize,
    pub p: usize,
    pub etype: EdgeType
}

impl Edge {

    pub fn is_unk(&self) -> bool {
        self.etype == EdgeType::Unk
    }
    
    pub fn better_than(&self, o: &Edge) -> bool {
        if self.unk < o.unk {
            return true
        }

        if self.unk > o.unk {
            return false
        }

        if self.w < o.w {
            return true
        }

        if self.w > o.w {
            return false
        }

        if o.is_unk() && !self.is_unk() {
            return true
        }

        return false
    }

    pub fn better(a :&Option<Edge>, b:&Option<Edge>) -> bool {
        if a.is_none() {
            return false
        }

        if b.is_none() {
            return true
        }

        return a.unwrap().better_than(&b.unwrap());
    }
}
