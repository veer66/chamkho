# Chamkho
Thai word segmentation library in Rust

# Usage

## As library

```rust
extern crate chamkho;

use chamkho::wordcut::Wordcut;
use chamkho::dict::Dict;

fn main() { 
     let dict = Dict::load(Dict::default_path());
     let wordcut = Wordcut::new(dict.unwrap());
     let segmented_string = wordcut.put_delimiters(&"กากกา".to_string(), "|");
     println!("{}", segmented_string);
}
```

## As command line

    echo "กากกา" | wordcut 
