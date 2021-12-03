use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use jieba_rs::Jieba;
use walkdir::{DirEntry, WalkDir};

fn is_markdown(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".md"))
        .unwrap_or(false)
}

fn main() {
    count_by_path(PathBuf::from("_fixtures").join("blog"));
}

fn count_by_path(path: PathBuf) {
    let mut count: HashMap<String, usize> = HashMap::new();
    let jieba = Jieba::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if is_markdown(&entry) {
            let content = fs::read_to_string(entry.path()).unwrap();
            let x = content.as_str();
            let results = jieba.cut(x, true);
            let mut one_count: HashMap<String, usize> = HashMap::new();
            for result in results {
                *one_count.entry(String::from(result)).or_insert(0) += 1;
                *count.entry(String::from(result)).or_insert(0) += 1;
            }

            let mut count_vec: Vec<_> = one_count.iter().collect();
            count_vec.sort_by(|a, b| a.1.cmp(b.1));
            println!("{:?}", count_vec);
        }
    }

    // let mut count_vec: Vec<_> = count.iter().collect();
    // count_vec.sort_by(|a, b| a.1.cmp(b.1));
}
