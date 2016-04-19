// Brief implementation of the 'node-google'
// node.js package.

extern crate hyper;
extern crate select;
extern crate regex;

use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;

use self::select::document::Document;
use self::select::predicate::Class;
use self::select::predicate::Name;
use self::select::predicate::And;

use self::regex::Regex;


#[derive(Debug)]
pub struct GoogleResult {
    link: String,
}

impl GoogleResult {
    pub fn new(link: &str) -> Self {
        GoogleResult { link: String::from(link) }
    }
}


pub struct Google {
    per_page: usize,
}

impl Default for Google {
    fn default() -> Self {
        Google { per_page: 10 }
    }
}


impl Google {
    pub fn new(results_per_page: usize) -> Self {
        let mut per_page = results_per_page;
        if per_page > 100 {
            per_page = 100;
        }

        Google { per_page: per_page }
    }

    pub fn google(&self, query: &str) -> Vec<GoogleResult> {
        let body = self.request(query);
        self.get_links(&body)
    }

    fn request(&self, query: &str) -> String {
        let url = format!("https://www.google.\
                           com/search?hl=en&q={}&start=0&sa=N&num={}&ie=UTF-8&oe=UTF-8&gws_rd=ssl",
                          query,
                          self.per_page);

        let client = Client::new();
        let mut res = client.get(&url)
                            .header(Connection::close())
                            .send()
                            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        body
    }

    fn get_links(&self, body: &str) -> Vec<GoogleResult> {
        let doc = Document::from(body);

        let item_selector = And(Name("div"), Class("g"));
        let link_container_selector = And(Name("h3"), Class("r"));
        let link_selector = Name("a");
        let cleaning_regex = Regex::new(r"(/url\?q=)(?P<link>.*)&(sa=.*&?)(ved=.*&?)(usg=.*&?)")
                                 .unwrap();

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

            let link_node = link_container.find(link_selector).first().unwrap();
            let link_href = link_node.attr("href").unwrap();

            if link_href.contains("/search?q=") {
                continue;
            }

            let clean_link = cleaning_regex.captures(&link_href)
                                           .unwrap()
                                           .name("link")
                                           .unwrap();
            results.push(GoogleResult::new(&clean_link));
        }

        results
    }
}
