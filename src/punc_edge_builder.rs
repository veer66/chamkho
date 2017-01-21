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
use std::collections::HashSet;
use edge::EdgeType;
use pat_edge_builder::PatEdgeBuilder;

lazy_static! {
    static ref PUNC_SET: HashSet<char> = {
        let mut m = HashSet::new();
        m.insert(' ');
        m.insert('\n');
        m.insert('\t');
        m.insert('\r');
        m.insert('"');
        m.insert('(');
        m.insert(')');
        m.insert('“');
        m.insert('”');
        m
    };
}

fn is_punc(ch: char) -> bool {
    PUNC_SET.contains(&ch)
}

pub fn create_punc_edge_builder() -> PatEdgeBuilder {
    PatEdgeBuilder::new(Box::new(is_punc), EdgeType::Punc)
}
