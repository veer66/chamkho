#![feature(io)]
    
mod dict;
mod acc;
mod graph;
use dict::Dict;
use graph::Graph;

fn main() {
    let _dict = dict::Dict::load("tdict-std.txt").unwrap();
    let g = Graph::build("กามกา", &_dict);
    println!("@@@@ {:?}", g.to_str_vec());
}