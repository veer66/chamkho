#![feature(io)]
#![feature(old_io)]
    
use dict::Dict;
use graph::Graph;
use std::old_io as io;

fn main() {
    let _dict = Dict::load("tdict-std.txt").unwrap();
    let mut reader = io::stdin();
    let mut o = io::stdout();
    loop {
        let input = match reader.read_line() {
            Ok(line) => line,
            Err(_) => break
        };        
        let g = Graph::build((&input[..]).trim_right_matches('\n'), &_dict);
        let segmented: &str = &(g.to_str_vec().connect("|"))[..];
        o.write_str(segmented);
        o.write_str("\n");
    }
} 
