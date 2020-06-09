#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate norvig_spell_checker;
extern crate rayon;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::convert::Infallible;
//use futures::{future, Future};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};

use clap::{App, Arg, ArgMatches, SubCommand};
use command_line_corrections::provide_words_corrections;
use dotenv::dotenv;
use std::env;
use futures::{StreamExt, TryStreamExt};

const CMD_RUN: &str = "run";
const CMD_CORRECT: &str = "correct";

mod command_line_corrections;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
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
            let (corpus_file, words) = correct_cmd_handler(&matches);
            provide_words_corrections(corpus_file, words);
            Ok(())
        }
        (CMD_RUN, _) => {
            let make_svc = make_service_fn(|_conn| {
                // This is the `Service` that will handle the connection.
                // `service_fn` is a helper to convert a function that
                // returns a Response into a `Service`.
                async { Ok::<_, Infallible>(service_fn(microservice_handler)) }
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

fn correct_cmd_handler<'a>(matches: &'a ArgMatches) -> (String, Vec<&'a str>) {
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

async fn microservice_handler(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {

    match (req.method(), req.uri().path()){
        (&Method::GET, "/correction") => {

            let mut body = Vec::new();
            while let Some(chunk) = req.body_mut().next().await {
                body.extend_from_slice(&chunk.unwrap());
            }
            // try to parse as json with serde_json
            let correction_req: CorrectionRequest = serde_json::from_slice(&body).unwrap();
            info!("Received {:?}", correction_req);

            let serialized_correction_req = serde_json::to_string(&correction_req).unwrap();
            Ok(Response::new(Body::from(serialized_correction_req)))
        }
        _ => {
            Ok(empty_response_with_code(StatusCode::NOT_FOUND))
        }
    }
}


fn empty_response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}

#[derive(Debug)]
#[derive(Deserialize)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CorrectionRequest
{
    words: Vec<String>
}

struct Correction
{
    word: String,
    correction: String
}

struct CorrectionResponse
{
    corrections: Vec<Correction>
}