mod google;
mod stackexchange;

extern crate regex;
extern crate hyper;
extern crate rustc_serialize;
extern crate select;
extern crate flate2;
extern crate getopts;

use std::env;

use getopts::Options;

use google::Google;
use stackexchange::StackExchangeApi;


fn print_usage(opts: Options) {
    let brief = "Usage: how2-rs [options] query";
    print!("{}", opts.usage(&brief));
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("m",
                "max-answers",
                "Maximum answers to retrieve (defaults to 5).",
                "COUNT");
    opts.optflag("h", "help", "Show usage.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let max_answers_count;
    if matches.opt_present("m") {
        let count_cli = matches.opt_str("m");
        match count_cli {
            Some(count_str) => {
                match count_str.parse::<usize>() {
                    Ok(count) => {
                        max_answers_count = count;
                    }
                    Err(_) => {
                        println!("'m' option should be an integer.");
                        return;
                    }
                }
            }
            None => {
                println!("Argument to option 'm' missing.");
                return;
            }
        }
    } else {
        max_answers_count = 5;
    }

    let query = matches.free.join(" ");

    let google = Google::default();
    let links = google.google(&query);
    let links_urls = links.iter()
                          .map(|l| l.link.as_str())
                          .collect();

    let api = StackExchangeApi::new();

    let mut answers = api.answers(links_urls);

    answers.sort_by_key(|a| -a.score);
    let left_answers: Vec<_> = answers.iter().take(max_answers_count).collect();

    for answer in left_answers.iter() {
        println!("{}", answer);
        println!("===-----===");
    }
}
