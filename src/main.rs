mod google;
mod stackexchange;

extern crate regex;
extern crate hyper;
extern crate rustc_serialize;
extern crate select;
extern crate flate2;

use google::Google;
use stackexchange::StackExchangeApi;

fn main() {
    let google = Google::default();
    let links = google.google("python unittest skip base class");

    let api = StackExchangeApi::new();

    let mut answers = Vec::new();
    for link in links.iter() {
        match api.answers(&link.link) {
            Ok(ref mut link_answers) => answers.append(link_answers),
            Err(text) => {
                println!("{}", text);
            }
        }
    }


    println!("{:?}", answers);
}
