#[macro_use]
extern crate log;
extern crate env_logger;

extern crate norvig_spell_checker;
extern crate regex;


use std::time::{Instant};

fn main() {
    // let re = regex::Regex::new(r"(?P<word>\w+)").unwrap();
    //
    // let line = "*** START OF THE PROJECT GUTENBERG EBOOK, THE ADVENTURES OF SHERLOCK HOLMES ***";
    // let words: Vec<String> = re
    //     .captures_iter(line)
    //     .map(|cap| { (&cap["word"]).to_ascii_lowercase() })
    //     .collect();
    // for word in &words {
    //     println!("{}", word);
    // }
    // //println!("{}", type_of(words));
    // let counts = counter(&words);
    // for (k, v) in counts.iter() {
    //     println!("{}: {}", k, v);
    // }
    env_logger::init();
    let now = Instant::now();
    let file: String = String::from("/home/gsiekaniec/devel/rust_projects/norvig_spell_checker/data/big.txt");
    let sc = norvig_spell_checker::spell_checker::SpellChecker::from_corpus_file(file);
    let word = String::from("peotryy");
    println!("Correction: {}", sc.correction(&word));
    info!("Bla");

    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
}