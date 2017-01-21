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
#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;


mod dict;
mod edge;
mod edge_builder;
mod dict_edge_builder;
mod unk_edge_builder;
mod pat_edge_builder;
mod punc_edge_builder;
mod latin_edge_builder;
mod graph_builder;
mod graph;
mod wordcut;
use std::io;
use std::io::BufRead;
use wordcut::Wordcut;
use dict::Dict;
use clap::App;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let lang = matches.value_of("lang");
    let dict_path = match matches.value_of("dict_path") {
        Some(_dict_path) => Path::new(_dict_path),
        None => {
            match lang {
                Some("lao") => Dict::lao_path(),
                Some("thai") | Some(_) | None => Dict::default_path()
            }            
        }
    };
    let word_delim = match matches.value_of("word_delimiter") {
        Some(word_delim) => word_delim,
        None => "|"
    };
    let dict = Dict::load(dict_path);
    let wordcut = Wordcut::new(dict.unwrap());
    
    for line_opt in io::BufReader::new(io::stdin()).lines() {

        let cleaned_line = match line_opt {
            Ok(line) => if line.len() > 0 {
                line.trim_right_matches('\n').to_string()
            } else {
                line
            },
            Err(e) => panic!("Cannot read line {}", e)
        };

        let segmented_string = wordcut.put_delimiters(&cleaned_line, word_delim);
        println!("{}", segmented_string);
    }
}
