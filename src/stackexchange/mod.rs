mod data;

use std::io::Read;
use std::collections::BTreeMap;

use super::regex::Regex;

use super::hyper::Client;
use super::hyper::header::Connection;

use super::rustc_serialize::json;
use super::rustc_serialize::json::Json;

use super::flate2::read::GzDecoder;

pub use self::data::StackExchangeAnswer;
use self::data::StackExchangeQuestion;


pub struct StackExchangeApi {
    api: &'static str,
    protocol: &'static str,
    version: &'static str,
    client: Client,
}


impl StackExchangeApi {
    pub fn new() -> Self {
        StackExchangeApi {
            api: "api.stackexchange.com",
            protocol: "https",
            version: "2.2",
            client: Client::new(),
        }
    }

    pub fn answers(&self, urls: Vec<&str>) -> Vec<StackExchangeAnswer> {
        let questions = self.parse_questions(urls);
        self.query(questions)
    }

    fn parse_questions(&self, urls: Vec<&str>) -> Vec<StackExchangeQuestion> {
        let mut questions = Vec::new();

        for url in urls.iter() {
            let question_id;
            let site;

            if url.contains("stackoverflow.com") {
                let re = Regex::new(r".*stackoverflow.com/questions/(?P<q_id>\d+)/")
                    .expect("Failed to initialize regex for question id retrieval");
                let captures = re.captures(&url).expect("Failed to match on URL");
                question_id = captures.name("q_id").expect("Failed to extract question id");
                site = "stackoverflow".into();
            } else if url.contains("stackexchange.com") {
                let re = Regex::new(r".*//(?P<site>.*).stackexchange.com/questions/(?P<q_id>\d+)/")
                    .expect("Failed to initialize regex for question id retrieval");
                let captures = re.captures(&url).expect("Failed to match on URL");
                question_id = captures.name("q_id").expect("Failed to retrieve question id");
                site = captures.name("site").expect("Failed to retrieve site name");
            } else {
                continue;
            }

            let question_id: u64 = question_id.parse::<u64>()
                .expect("Question id is not an integer");

            questions.push(StackExchangeQuestion::new(question_id, site));
        }
        questions
    }

    fn request(&self, url: &str) -> String {
        let mut res = self.client
            .get(url)
            .header(Connection::close())
            .send()
            .expect("Failed to send request to StackExchange");

        let mut buffer = Vec::new();
        res.read_to_end(&mut buffer).expect("Failed to read gzipped response from StackExchange");

        let mut gzip_decoder = GzDecoder::new(buffer.as_slice())
            .expect("Failed to decode gzipped response");
        let mut body = String::new();
        gzip_decoder.read_to_string(&mut body).expect("Failed to read response from StackExchange");

        body
    }

    fn from_json(&self, json_str: &str) -> Vec<StackExchangeAnswer> {
        let json = Json::from_str(&json_str).expect("Failed to encode StackExchange data to JSON");
        let data = json.as_object().expect("Conversion to map failed");
        let items = data.get("items")
            .expect("Missing key 'items'")
            .as_array()
            .expect("Failed to convert 'items' to array");

        let answers: Vec<StackExchangeAnswer> = items.iter()
            .map(|i| json::decode(&i.to_string()).expect("Failed to encode StackExchange answer"))
            .collect();

        answers
    }

    fn query(&self, questions: Vec<StackExchangeQuestion>) -> Vec<StackExchangeAnswer> {
        let mut answers = Vec::new();
        let mut by_site = BTreeMap::new();

        for q in questions.iter() {
            if !by_site.contains_key(&q.site) {
                let questions_same_site = questions.iter()
                    .filter(|que| que.site == q.site)
                    .collect::<Vec<_>>();
                by_site.insert(&q.site, questions_same_site);
            }
        }

        for (site, site_questions) in by_site.iter() {
            let question_ids: String = site_questions.iter()
                .map(|q| q.id.to_string())
                .collect::<Vec<_>>()
                .join(";");

            let url = format!("{}://{}/{}/questions/{}/answers/?site={}&filter=withbody",
                              self.protocol,
                              self.api,
                              self.version,
                              question_ids,
                              site);

            let response = self.request(&url);
            let mut new_answers = self.from_json(&response);
            answers.append(&mut new_answers);
        }

        answers
    }
}
