mod data;

use std::io::Read;

use super::regex::Regex;

use super::hyper::Client;
use super::hyper::header::Connection;

use super::rustc_serialize::json;
use super::rustc_serialize::json::Json;

use super::flate2::read::GzDecoder;

use google::GoogleResult;
use self::data::StackExchangeAnswerMeta;


pub struct StackExchangeApi {
    api: &'static str,
    version: &'static str,
    protocol: &'static str,
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

    pub fn fetch_answers(&self, url: &str) -> Result<Vec<StackExchangeAnswerMeta>, String> {
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

        Ok(self.query(site, quiestion_id))
    }

    fn query(&self, site: &str, quiestion_id: u64) -> Vec<StackExchangeAnswerMeta> {
        let url = format!("{}://{}/questions/{}/answers?site={}",
                          self.protocol,
                          self.api,
                          quiestion_id,
                          site);

        let mut res = self.client
                          .get(&url)
                          .header(Connection::close())
                          .send()
                          .unwrap();

        let mut buffer = Vec::new();
        res.read_to_end(&mut buffer);

        let mut gzip_decoder = GzDecoder::new(buffer.as_slice()).unwrap();
        let mut body = String::new();
        gzip_decoder.read_to_string(&mut body).unwrap();

        let response = Json::from_str(&body).unwrap();
        let data = response.as_object().unwrap();
        let items = data.get("items").unwrap().as_array().unwrap();

        let answers: Vec<StackExchangeAnswerMeta> = items.iter()
                                                         .map(|i| {
                                                             json::decode(&i.to_string()).unwrap()
                                                         })
                                                         .collect();

        answers
    }
}
