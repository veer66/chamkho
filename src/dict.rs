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

extern crate prefixtree;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use self::prefixtree::PrefixTree;

#[derive(Clone)]
pub struct Dict {
    prefix_tree: PrefixTree<bool>,
}

impl Dict {
    pub fn default_path() -> &'static Path {
        Path::new(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/data/thai.txt"))
    }

    pub fn lao_path() -> &'static Path {
        Path::new(
            concat!(env!("CARGO_MANIFEST_DIR"),
                    "/data/laowords.txt"))
    }

    #[allow(dead_code)]
    pub fn create<'a>(words: &Vec<String>) -> Result<Dict, &'a str> {
        let words_payloads: Vec<(&str, bool)> =
            words.iter().map(|word| (&word[..], true)).collect();
        Ok(Dict{prefix_tree:PrefixTree::new(&words_payloads[..])})
    }

    pub fn load<'a>(path: &Path) -> Result<Dict, &'a str> {
        let mut f = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err("Cannot open dict")
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(_) => return Err("Cannot read dict")
        };
        let mut words: Vec<String> = s.lines().map(|word| String::from(word)).collect();
        words.sort();
        let words_payloads: Vec<(&str, bool)>
            = words.iter().map(|word| (&word[..], true)).collect();

        Ok(Dict{prefix_tree:PrefixTree::new(&words_payloads[..])})
    }

    pub fn seek(&self, p: usize, offset: usize, ch: char) -> Option<&(u32,bool,Option<bool>)> {
        self.prefix_tree.seek(&(p as u32, offset as u32, ch))
    }

}
