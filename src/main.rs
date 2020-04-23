extern crate norvig_spell_checker;
extern crate regex;

use std::time::{Duration, Instant};

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
    let now = Instant::now();
    let file: String = String::from("/home/gsiekaniec/devel/rust_projects/norvig_spell_checker/data/big.txt");
    let sc = norvig_spell_checker::spell_checker::SpellChecker::initialize(file);
    let word = String::from("stupid");
    println!("{}", sc.correction(word));

    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
}