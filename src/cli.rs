mod lib;

use clap::Parser;
use std::io;
use std::io::BufRead;
use std::path::Path;
use wordcut_engine::replacer;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Lang {
    Lao,
    Khmer,
    Myanmar,
    Thai,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    dict_path: Option<String>,
    #[clap(short, long, value_parser)]
    cluster_rules_path: Option<String>,
    #[clap(short = 's', long, value_parser)]
    word_delimiter: Option<String>,
    #[clap(short, long, value_parser, value_enum)]
    lang: Option<Lang>,
}

fn main() {
    let args = Args::parse();
    let lang = args.lang;
    let dict_path = match args.dict_path.as_ref() {
        Some(dict_path) => Path::new(dict_path),
        None => match lang {
            Some(Lang::Lao) => lib::lao_path(),
            Some(Lang::Khmer) => lib::khmer_dict_path(),
            Some(Lang::Myanmar) => lib::myanmar_dict_path(),
            Some(Lang::Thai) | None => lib::default_path(),
        },
    };
    let word_delim = match args.word_delimiter.as_ref().map(|delim| delim.as_str()) {
        Some(word_delim) => word_delim,
        None => "|",
    };
    let dict = lib::load_dict(dict_path).unwrap();

    let cluster_rule_path = if let Some(cluster_rules_path) = args.cluster_rules_path {
        Some(cluster_rules_path.to_string())
    } else {
        match lang {
            Some(Lang::Lao) => lib::lao_clusters_path(),
            Some(Lang::Khmer) => lib::khmer_clusters_path(),
            Some(Lang::Myanmar) => lib::myanmar_clusters_path(),
            Some(Lang::Thai) | None => lib::thai_cluster_path(),
        }
    };

    let wordcut = match cluster_rule_path {
        Some(cluster_rule_path) => {
            let cluster_re =
                lib::wordcut_engine::load_cluster_rules(Path::new(&cluster_rule_path)).unwrap();
            lib::Wordcut::new_with_cluster_re(dict, cluster_re)
        }
        None => lib::Wordcut::new(dict),
    };

    let replace_rules = match lang {
        Some(Lang::Thai) | None => lib::thai_replace_rules_path()
            .map(|path| replacer::load_imm_rules(&path).expect("Load replace rules"))
            .unwrap_or(vec![]),
        _ => vec![],
    };

    for line_opt in io::stdin().lock().lines() {
        let cleaned_line = match line_opt {
            Ok(line) => line.trim_end_matches('\n').to_string(),
            Err(e) => panic!("Cannot read line {}", e),
        };
        let mod_line = replacer::replace(&replace_rules, &cleaned_line);
        let segmented_string = wordcut.put_delimiters(&mod_line, word_delim);
        println!("{}", segmented_string);
    }
}
