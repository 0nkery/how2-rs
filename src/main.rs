mod google;
mod stackexchange;

extern crate regex;
extern crate hyper;
extern crate rustc_serialize;
extern crate select;
extern crate flate2;
extern crate getopts;

use std::env;
use std::process::exit;

use getopts::Options;

use rustc_serialize::json;

use google::Google;
use stackexchange::StackExchangeApi;
use stackexchange::StackExchangeAnswer;


#[derive(Debug)]
struct How2RSSettings {
    max_answers_count: usize,
    query: String,
    json: bool,
}

impl Default for How2RSSettings {
    fn default() -> Self {
        How2RSSettings {
            max_answers_count: 5,
            query: String::new(),
            json: false,
        }
    }
}


fn print_usage(opts: Options) {
    let brief = "Usage: how2-rs [options] query";
    print!("{}", opts.usage(&brief));
}


fn get_settings() -> How2RSSettings {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("m",
                "max-answers",
                "Maximum answers to retrieve (defaults to 5).",
                "COUNT");
    opts.optflag("j", "json", "Return answers json-encoded (defaults to false).");
    opts.optflag("h", "help", "Show usage.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        exit(0);
    }

    let mut settings = How2RSSettings::default();

    if matches.opt_present("j") {
        settings.json = true;
    }

    if matches.opt_present("m") {
        let count_cli = matches.opt_str("m");
        match count_cli {
            Some(count_str) => {
                match count_str.parse::<usize>() {
                    Ok(count) => {
                        settings.max_answers_count = count;
                    }
                    Err(_) => {
                        println!("'m' option should be an integer.");
                        exit(1);
                    }
                }
            }
            None => {
                println!("Argument to option 'm' missing.");
                exit(1);
            }
        }
    }

    settings.query = matches.free.join(" ");

    settings
}


fn search(settings: &How2RSSettings) -> Vec<StackExchangeAnswer> {
    let google = Google::default();
    let links = google.google(&settings.query);
    let links_urls = links.iter()
                          .map(|l| l.link.as_str())
                          .collect();

    let api = StackExchangeApi::new();

    let mut answers = api.answers(links_urls);

    answers.sort_by_key(|a| -a.score);
    let left_answers: Vec<_> = answers.drain(..)
                                      .take(settings.max_answers_count)
                                      .collect();

    left_answers
}


fn main() {
    let settings = get_settings();
    let answers = search(&settings);

    if settings.json == true {
        let encoded = json::encode(&answers).unwrap();
        println!("{}", encoded);
    } else {
        for answer in answers.iter() {
            println!("{}", answer);
            println!("===-----===");
        }
    }
}


#[test]
fn stackexchange_works() {
    let mut settings = How2RSSettings::default();
    settings.query = String::from("rust lang linked list");

    let answers = search(settings);

    assert_eq!(answers.len(), 3);
    assert!(answers.iter().any(|a| a.answer_id == 22269167));
}

#[test]
fn no_bad_results() {
    let mut settings = How2RSSettings::default();
    settings.query = String::from("rust lang is not awesome");

    let answers = search(settings);

    assert_eq!(answers.len(), 0);
}
