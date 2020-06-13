#[macro_use]
extern crate log;
extern crate norvig_spell_checker;
extern crate env_logger;
extern crate hyper;
extern crate rayon;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use clap::{App, Arg, ArgMatches, SubCommand};
use dotenv::dotenv;
use std::env;
use futures::{StreamExt};
use norvig_spell_checker::spell_checker::SpellChecker;
use std::sync::Arc;
use crate::command_line_corrections::{CorrectionResponse, CorrectionRequest,
                                      find_words_corrections, print_correction};

const CMD_RUN: &str = "run";
const CMD_CORRECT: &str = "correct";

mod command_line_corrections;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    dotenv().expect("Failed to read .env file");
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
        .subcommand(SubCommand::with_name(CMD_RUN)
            .about("Run the correction server.")
            .arg(
                Arg::with_name("corpus")
                    .help("Specifies a corpus file to initialise spell-checker")
                    .takes_value(true)
                    .short("c")
                    .long("corpus")
                    .required(false)
                    .multiple(false),
            )
        )
        .get_matches();

    match matches.subcommand() {
        (CMD_CORRECT, Some(matches)) => {
            let (corpus_file, words) = cli_correction_handler(&matches);
            let spell_checker = SpellChecker::from_corpus_file_par(&corpus_file);
            let word_and_corrections = find_words_corrections(&spell_checker, words);
            let word_and_correction_vec
                = word_and_corrections.corrections
                .into_iter()
                .map(|correction| vec![correction.word, correction.correction])
                .collect();
            print_correction(&word_and_correction_vec);
            Ok(())
        }
        (CMD_RUN, Some(matches)) => {
            let corpus_file = corpus_file(matches);
            let spell_checker = Arc::new(
                SpellChecker::from_corpus_file_par(&corpus_file)
            );

            let make_svc = make_service_fn(move |_conn| {
                // This is the `Service` that will handle the connection.
                // `service_fn` is a helper to convert a function that
                // returns a Response into a `Service`.
                let spell_checker = spell_checker.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                        let spell_checker = spell_checker.clone();
                            microservice_handler(req, spell_checker)
                    }))
                }
            });

            let addr = env::var("ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:8080".into())
                .parse()
                .expect("can't parse ADDRESS variable");

            let server = Server::bind(&addr).serve(make_svc);

            info!("Listening on http://{}", addr);

            server.await?;

            Ok(())
        }
        _ => {
            matches.usage();
            Ok(())
        }
    }
}

fn cli_correction_handler(matches: &ArgMatches) -> (String, Vec<String>) {
    let corpus_file = corpus_file(matches);
    let words: Vec<String> = {
        let arg_words = matches.values_of("words");
        if arg_words.is_some() {
            arg_words
                .unwrap()
                .into_iter()
                .map(|word| word.to_string())
                .collect()
        } else {
            vec![]
        }
    };

    (corpus_file, words)
}

fn corpus_file(matches: &ArgMatches) -> String {
    let corpus_file: String = {
        let corpus_arg = matches.value_of("corpus");
        if corpus_arg.is_some() {
            corpus_arg.unwrap().to_string()
        } else {
            info!("Using default corpus file");
            String::from("/home/gsiekaniec/devel/rust_projects/norvig_spell_checker/data/big.txt")
        }
    };
    corpus_file
}

async fn microservice_handler(mut req: Request<Body>, spell_checker: Arc<SpellChecker>)
    -> Result<Response<Body>, Infallible> {

    match (req.method(), req.uri().path()){
        (&Method::GET, "/correction") => {
            let response = handle_get_correction_request(&mut req, spell_checker).await;
            let serialized_response = serde_json::to_string(&response).unwrap();
            Ok(Response::new(Body::from(serialized_response)))
        }
        _ => {
            Ok(empty_response_with_code(StatusCode::NOT_FOUND))
        }
    }
}

async fn handle_get_correction_request(req: &mut Request<Body>, spell_checker: Arc<SpellChecker>)
    -> CorrectionResponse {

    let mut body = Vec::new();
    while let Some(chunk) = req.body_mut().next().await {
        body.extend_from_slice(&chunk.unwrap());
    }
    // TODO: add handling in case parsing fails
    let correction_req: CorrectionRequest = serde_json::from_slice(&body).unwrap();
    info!("Received {:?}", correction_req);
    find_words_corrections(&spell_checker, correction_req.words)
}


fn empty_response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}


