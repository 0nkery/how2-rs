use std::fmt;

use regex::Regex;

#[derive(RustcDecodable, Debug)]
pub struct StackExchangeUser {
    pub reputation: i32,
    pub accept_rate: Option<i32>,
}


#[derive(RustcDecodable, Debug)]
pub struct StackExchangeAnswer {
    pub owner: StackExchangeUser,
    pub is_accepted: bool,
    pub score: i32,
    pub answer_id: u64,
    pub body: Option<String>,
}


impl StackExchangeAnswer {
    fn simple_print(&self) -> String {
        match self.body {
            None => String::new(),
            Some(ref body) => {
                let re = Regex::new(r"<.*>").unwrap();
                re.replace_all(&body, "")
            }
        }
    }
}

impl fmt::Display for StackExchangeAnswer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.simple_print())
    }
}
