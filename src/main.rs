mod google;
mod stackexchange;

extern crate regex;
extern crate hyper;
extern crate rustc_serialize;
extern crate select;
extern crate flate2;

use google::Google;
use google::GoogleResult;
use stackexchange::StackExchangeApi;

fn main() {
    // let google = Google::default();
    // let links = google.google("rust");

    // println!("{:?}", links);

    let api = StackExchangeApi::new();
    let answers = api.fetch_answers("http://stackoverflow.\
                                     com/questions/36713164/list-all-photo-albums-in-ios");

    println!("{:?}", answers);
}
