mod dict;
mod edge;
mod graph_builder;
mod graph;
mod acc;
mod space_acc;
mod wordcut;
use std::io;
use std::io::BufRead;
use wordcut::Wordcut;
use dict::Dict;

fn main() {
    let dict = Dict::load_default();
    let wordcut = Wordcut::new(dict.unwrap());

    for line in io::BufReader::new(io::stdin()).lines() {
        let cleaned_line = line.unwrap().trim_right_matches('\n').to_string();
        let segmented_string = wordcut.put_delimiters(&cleaned_line, "|");
        println!("{}", segmented_string);
    }
}