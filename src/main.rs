#[macro_use]
extern crate log;
extern crate env_logger;
extern crate norvig_spell_checker;
extern crate rayon;

use ascii_table::{Align, AsciiTable, Column};
use clap::{App, Arg, ArgMatches, SubCommand};
use rayon::prelude::*;
use std::time::Instant;

const CMD_RUN: &str = "run";
const CMD_CORRECT: &str = "correct";

fn print_correction(word_correction: &Vec<Vec<String>>) {
    let mut ascii_table = AsciiTable::default();
    let mut word_column = Column::default();
    word_column.header = "Word".into();
    word_column.align = Align::Left;
    ascii_table.columns.insert(0, word_column);

    let mut suggestion_column = Column::default();
    suggestion_column.header = "Correction".into();
    suggestion_column.align = Align::Left;
    ascii_table.columns.insert(1, suggestion_column);

    ascii_table.print(word_correction);
}

fn main() {
    let matches = App::new("spell-checker")
        .version("1.0")
        .author("Grzegorz Siekaniec")
        .about("Suggests correction for a passed word or list of words")
        .subcommand(
            SubCommand::with_name(CMD_CORRECT)
                .about("Provide corrections for specified words")
                .arg(
                    Arg::with_name("corpus")
                        .help("Specifies a corpus file to initialise spell-checker")
                        .takes_value(true)
                        .short("c")
                        .long("corpus")
                        .required(false)
                        .multiple(false),
                )
                .arg(Arg::with_name("words").required(true).multiple(true)),
        )
        .subcommand(SubCommand::with_name(CMD_RUN).about("Run the correction server."))
        .get_matches();

    env_logger::init();
    match matches.subcommand() {
        (CMD_CORRECT, Some(matches)) => {
            let (corpus_file, words) = extract_correct_cmd_arguments(&matches);
            provide_words_corrections(corpus_file, words);
        }
        (CMD_RUN, _) => {
            // start server
        }
        _ => {
            matches.usage();
        }
    }
}

fn extract_correct_cmd_arguments<'a>(matches: &'a ArgMatches) -> (String, Vec<&'a str>) {
    let corpus_file: String = {
        let corpus_arg = matches.value_of("corpus");
        if corpus_arg.is_some() {
            corpus_arg.unwrap().to_string()
        } else {
            info!("Using default corpus file");
            String::from("/home/gsiekaniec/devel/rust_projects/norvig_spell_checker/data/big.txt")
        }
    };
    let words: Vec<&str> = {
        let arg_words = matches.values_of("words");
        if arg_words.is_some() {
            arg_words.unwrap().collect()
        } else {
            vec![]
        }
    };

    (corpus_file, words)
}

fn provide_words_corrections(corpus_file: String, words: Vec<&str>) {
    info!("Using corpus file located at {:}", corpus_file);
    info!("Words {:?}", words);
    let now = Instant::now();
    let sc = norvig_spell_checker::spell_checker::SpellChecker::from_corpus_file_par(&corpus_file);
    // let hm_par = sc.word_count.clone();
    //
    // let sc = norvig_spell_checker::spell_checker::SpellChecker::from_corpus_file(&corpus_file);
    // let hm_seq = sc.word_count.clone();

    // assert_eq!(hm_par, hm_seq);

    let word_correction: Vec<Vec<_>> = words
        .par_iter()
        .map(|word| {
            let word = word.to_string();
            let correction = sc.correction(&word);
            vec![word, correction]
        })
        .collect();

    print_correction(&word_correction);

    let new_now = Instant::now();
    info!(
        "It took {:?} to find corrections for words",
        new_now.duration_since(now)
    );
}
