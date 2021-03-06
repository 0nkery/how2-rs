// Brief implementation of the 'node-google'
// node.js package.
use std::io::Read;

use super::hyper::Client;
use super::hyper::header::Connection;

use super::select::document::Document;
use super::select::predicate::Class;
use super::select::predicate::Name;
use super::select::predicate::And;

use super::regex::Regex;

#[derive(Debug)]
pub struct GoogleResult {
    pub link: String,
}

impl GoogleResult {
    pub fn new(link: &str) -> Self {
        GoogleResult { link: String::from(link) }
    }
}


#[derive(Debug)]
pub struct Google {
    per_page: usize,
    client: Client,
}

impl Default for Google {
    fn default() -> Self {
        Google {
            per_page: 10,
            client: Client::new(),
        }
    }
}


impl Google {
    pub fn google(&self, query: &str) -> Vec<GoogleResult> {
        let body = self.request(query);
        self.get_links(&body)
    }

    pub fn per_page(&self) -> usize {
        self.per_page
    }

    fn request(&self, query: &str) -> String {
        let url = format!("https://www.google.\
                           com/search?hl=en&q={}&start=0&sa=N&num={}&ie=UTF-8&oe=UTF-8&gws_rd=ssl",
                          query,
                          self.per_page);

        let mut res = self.client
            .get(&url)
            .header(Connection::close())
            .send()
            .expect("Failed to query Google");

        let mut body = String::new();
        res.read_to_string(&mut body).expect("Failed to read response from Google");

        body
    }

    fn get_links(&self, body: &str) -> Vec<GoogleResult> {
        let doc = Document::from(body);

        let item_selector = And(Name("div"), Class("g"));
        let link_container_selector = And(Name("h3"), Class("r"));
        let link_selector = Name("a");
        let cleaning_regex = Regex::new(r"(/url\?q=)(?P<link>.*)&(sa=.*&?)(ved=.*&?)(usg=.*&?)")
            .expect("Failed to initialize cleaning Regex");

        let mut results = Vec::new();

        for item in doc.find(item_selector).iter().take(self.per_page) {
            let link_container;
            match item.find(link_container_selector).first() {
                Some(container) => {
                    link_container = container;
                }
                None => {
                    continue;
                }
            };

            let link_node =
                link_container.find(link_selector).first().expect("Failed to find 'a' node");
            let link_href = link_node.attr("href").expect("Node 'a' has no 'href' attribute");

            if link_href.contains("/search?q=") {
                continue;
            }

            let captures = cleaning_regex.captures(&link_href);
            let clean_link: &str;

            match captures {
                Some(cap) => {
                    clean_link = cap.name("link").expect("Failed to extract link");
                }
                None => continue,
            }

            results.push(GoogleResult::new(&clean_link));
        }

        results
    }
}


#[test]
fn google_works() {
    let google = Google::default();
    let results = google.google("rust lang testing");

    assert_eq!(results.len(), google.per_page());
    assert!(results.iter().any(|r| r.link == "https://doc.rust-lang.org/book/testing.html"));
}
