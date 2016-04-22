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
        if !self.body.is_some() {
            return String::new();
        }

        String::new()
    }
}
