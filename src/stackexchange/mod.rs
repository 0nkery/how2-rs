mod data;

use std::io::Read;

use super::regex::Regex;

use super::hyper::Client;
use super::hyper::header::Connection;

use super::rustc_serialize::json;
use super::rustc_serialize::json::Json;

use super::flate2::read::GzDecoder;

use self::data::StackExchangeAnswer;


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

    pub fn answers(&self, url: &str) -> Result<Vec<StackExchangeAnswer>, String> {
        let quiestion_id: &str;
        let site: &str;

        if url.contains("stackoverflow.com") {
            let re = Regex::new(r".*stackoverflow.com/questions/(?P<q_id>\d+)/").unwrap();
            let captures = re.captures(&url).unwrap();
            quiestion_id = captures.name("q_id").unwrap();
            site = "stackoverflow".into();
        } else if url.contains("stackexchange.com") {
            let re = Regex::new(r".*//(?P<site>.*).stackexchange.com/questions/(?P<q_id>\d+)/")
                         .unwrap();
            let captures = re.captures(&url).unwrap();
            quiestion_id = captures.name("q_id").unwrap();
            site = captures.name("site").unwrap();
        } else {
            return Err("Result is not related to StackExchange.".into());
        }

        let quiestion_id: u64 = quiestion_id.parse::<u64>().unwrap();

        Ok(self.query(site, quiestion_id, 20))
    }

    fn request(&self, url: &str) -> String {
        let mut res = self.client
                          .get(url)
                          .header(Connection::close())
                          .send()
                          .unwrap();

        let mut buffer = Vec::new();
        res.read_to_end(&mut buffer).unwrap();

        let mut gzip_decoder = GzDecoder::new(buffer.as_slice()).unwrap();
        let mut body = String::new();
        gzip_decoder.read_to_string(&mut body).unwrap();

        body
    }

    fn from_json(&self, json_str: &str) -> Vec<StackExchangeAnswer> {
        let json = Json::from_str(&json_str).unwrap();
        let data = json.as_object().unwrap();
        let items = data.get("items").unwrap().as_array().unwrap();

        let answers: Vec<StackExchangeAnswer> = items.iter()
                                                     .map(|i| {
                                                         json::decode(&i.to_string()).unwrap()
                                                     })
                                                     .collect();

        answers
    }

    fn query_meta(&self,
                  site: &str,
                  quiestion_id: u64,
                  max_count: usize)
                  -> Vec<StackExchangeAnswer> {
        let url = format!("{}://{}/{}/questions/{}/answers?site={}",
                          self.protocol,
                          self.api,
                          self.version,
                          quiestion_id,
                          site);

        let response = self.request(&url);
        let mut answers = self.from_json(&response);
        answers.sort_by_key(|a| -a.score);

        let chosen_answers = answers.drain(0..)
                                    .filter(|a| a.score > 0)
                                    .take(max_count)
                                    .collect();

        chosen_answers
    }

    fn query(&self, site: &str, quiestion_id: u64, max_count: usize) -> Vec<StackExchangeAnswer> {
        let answers_meta = self.query_meta(site, quiestion_id, max_count);

        let answer_ids = answers_meta.iter()
                                     .map(|a| a.answer_id.to_string())
                                     .collect::<Vec<_>>()
                                     .join(";");

        let url = format!("{}://{}/{}/answers/{}/?site={}&filter=withbody",
                          self.protocol,
                          self.api,
                          self.version,
                          answer_ids,
                          site);

        let response = self.request(&url);
        let mut answers = self.from_json(&response);
        answers.sort_by_key(|a| -a.score);

        answers
    }
}
