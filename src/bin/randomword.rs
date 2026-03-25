use std::path::PathBuf;

use rustytype::textgen::{RawWordSelector, WordSelector};
use rustytype::wordlists::OS_WORDLIST_PATH;

fn main() {
    let mut word_selector = RawWordSelector::from_path(PathBuf::from(OS_WORDLIST_PATH)).unwrap();

    let word = word_selector.new_word().unwrap();
    println!("{}", word);
}
