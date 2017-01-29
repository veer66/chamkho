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

use dict::Dict;
use graph::{Graph, TextRange};

#[derive(Clone)]
pub struct Wordcut {
    dict: Dict,
}

impl Wordcut {
    pub fn new(dict: Dict) -> Wordcut {
        Wordcut{dict: dict}
    }

    #[allow(dead_code)]
    pub fn segment(&self, txt: &String) -> Vec<TextRange> {
        let g = Graph::build(&txt[..], &self.dict);
        g.to_ranges()
    }

    pub fn segment_into_strings(&self, txt: &String) -> Vec<String> {
        let g = Graph::build(&txt[..], &self.dict);
        g.to_str_vec()
    }

    pub fn put_delimiters(&self, txt: &String, delim: &str) -> String {
        self.segment_into_strings(txt).join(delim)
    }
}
