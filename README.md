# Chamkho
Lao/Thai word segmentation library in Rust

# Usage

## As library

### Lao
```rust
extern crate chamkho;

use chamkho::wordcut::Wordcut;
use chamkho::dict::Dict;

fn main() { 
     let dict = Dict::load(Dict::lao_path());
     let wordcut = Wordcut::new(dict.unwrap());
     let segmented_string = wordcut.put_delimiters(&"ພາສາລາວມີ".to_string(), "|");
     println!("{}", segmented_string);
}
```


### Thai
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

### External dictionary

    echo "กากกา" | wordcut -d <path to dictionary>

### Specific language

    echo "ພາສາລາວມີ" | wordcut -l lao
