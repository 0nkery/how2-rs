use std::fmt;

use regex::Regex;


#[derive(Debug)]
pub struct StackExchangeQuestion {
    pub id: u64,
    pub site: String,
}


impl StackExchangeQuestion {
    pub fn new(id: u64, site: &str) -> Self {
        StackExchangeQuestion {
            id: id,
            site: site.into(),
        }
    }
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct StackExchangeUser {
    pub reputation: i32,
    pub accept_rate: Option<i32>,
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct StackExchangeAnswer {
    pub owner: StackExchangeUser,
    pub is_accepted: bool,
    pub score: i32,
    pub answer_id: u64,
    pub body: Option<String>,
}


impl StackExchangeAnswer {
    pub fn simple_print(&self) -> String {
        match self.body {
            None => String::new(),
            Some(ref body) => {
                let re = Regex::new(r"<.+?>").unwrap();
                let non_html = re.replace_all(&body, "");
                let non_escaped = non_html.replace("&lt;", "<")
                                          .replace("&gt;", ">")
                                          .replace("&amp;", "&");

                non_escaped
            }
        }
    }
}

impl fmt::Display for StackExchangeAnswer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.simple_print())
    }
}
