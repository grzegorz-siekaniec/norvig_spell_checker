#[macro_use]
extern crate log;
extern crate env_logger;
extern crate norvig_spell_checker;
extern crate rayon;

use clap::{App, Arg, ArgMatches, SubCommand};
use command_line_corrections::provide_words_corrections;
use dotenv::dotenv;

const CMD_RUN: &str = "run";
const CMD_CORRECT: &str = "correct";

mod command_line_corrections;

fn main() {
    dotenv().ok();
    env_logger::init();

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

    match matches.subcommand() {
        (CMD_CORRECT, Some(matches)) => {
            let (corpus_file, words) = extract_cmd_correct_arguments(&matches);
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

fn extract_cmd_correct_arguments<'a>(matches: &'a ArgMatches) -> (String, Vec<&'a str>) {
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
