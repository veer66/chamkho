// chamkho - a word breaker written in Rust
// Copyright (C) 2015  Vee Satayamas
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
//

use edge::{Edge, EdgeType};
use edge_builder::{EdgeBuilder, EdgeBuildingContext};
use dict::Dict;

#[derive(Clone)]
struct Pointer {
    node_id: usize,
    s: usize,
    offset: usize,
    is_final: bool,
}

impl Pointer {
    fn update(&mut self, dict: &Dict, ch: char) -> bool {
        match dict.seek(self.node_id, self.offset, ch) {
            None => false,
            Some(&(child_id, is_final, _)) => {
                self.node_id = child_id as usize;
                self.is_final = is_final;
                self.offset += 1;
                true
            }
        }
    }

    fn gen_edge(&self, path: &[Edge]) -> Edge {
        let source = path[self.s];
        Edge{etype: EdgeType::Dict,
             p: self.s,
             w: source.w + 1,
             unk: source.unk}
    }
}

pub struct DictEdgeBuilder<'a> {
    dict: &'a Dict,
    pointers: Vec<Pointer>,
}

impl<'a> DictEdgeBuilder<'a> {
    pub fn new(dict: &Dict) -> DictEdgeBuilder {
        const MAX_SIZE: usize = 0xFF;
        DictEdgeBuilder {
            dict: dict,
            pointers: Vec::with_capacity(MAX_SIZE)
        }
    }

    fn add_pointer(&mut self, context: &EdgeBuildingContext) {
        self.pointers.push(Pointer {
            node_id: 0,
            offset: 0,
            is_final: false,
            s: context.i,
        });
    }

    fn update_pointers(&mut self, context: &EdgeBuildingContext) {
        let mut j = 0;
        for i in 0..self.pointers.len() {
            let valid = self.pointers[i].update(self.dict, context.ch);
            if valid {
                if j < i {
                    self.pointers[j] = self.pointers[i].clone()
                }
                j += 1
            }
        }
        self.pointers.truncate(j);
    }

    fn gen_edge(&self, pointers: &[Pointer], path: &[Edge]) -> Option<Edge> {
        let mut best_edge: Option<Edge> = None;
        for pointer in pointers {
            if pointer.is_final {
                let edge = pointer.gen_edge(path);
                if best_edge.is_none() {
                    best_edge = Some(edge)
                } else if edge.better_than(&best_edge.unwrap()) {
                    best_edge = Some(edge)
                }
            }
        }
        return best_edge
    }
}

impl<'a> EdgeBuilder for DictEdgeBuilder<'a> {
    fn build(&mut self, context: &EdgeBuildingContext, path: &[Edge]) -> Option<Edge> {
        self.add_pointer(context);
        self.update_pointers(context);
        self.gen_edge(&self.pointers, path)
    }
}
