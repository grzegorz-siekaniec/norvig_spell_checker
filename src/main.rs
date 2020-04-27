#[macro_use]
extern crate log;
extern crate env_logger;

extern crate norvig_spell_checker;
extern crate regex;

use clap::clap_app;
use clap::{App, Arg};
use std::time::{Instant};

fn main() {
    let matches = App::new("spell-checker")
        .version("1.0")
        .author("Grzegorz Siekaniec")
        .about("Suggests correction for a passed word or list of words")
        .arg(//"-c, --corpus=<FILE> 'Specifies a corpus file to initialise spell-checker'"
             Arg::with_name("corpus")
                 .help("Specifies a corpus file to initialise spell-checker")
                 .takes_value(true)
                 .short("c")
                 .long("corpus")
                 .required(true)
                 .multiple(false)
        )
        .arg(
            Arg::with_name("words")
                .required(true)
                .multiple(true)
        )
        //.arg("<seq>... 'A sequence of whole positive numbers, i.e. 20 25 30'")
        .get_matches();

    env_logger::init();
    let file = matches.value_of("corpus");
    let words = matches.values_of("words");
    info!("Using corpus file from {:?}", file);
    info!("Words {:?}", words);
    let now = Instant::now();
    let file: String = String::from("/home/gsiekaniec/devel/rust_projects/norvig_spell_checker/data/big.txt");
    let sc = norvig_spell_checker::spell_checker::SpellChecker::from_corpus_file(file);
    let word = String::from("peotryy");
    println!("Correction: {}", sc.correction(&word));
    info!("Bla");

    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
}