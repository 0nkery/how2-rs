use super::super::rustc_serialize::Decodable;
use super::super::rustc_serialize::Decoder;
use super::super::rustc_serialize::json;


#[derive(RustcDecodable, Debug)]
pub struct StackExchangeUser {
    reputation: i32,
    user_type: String,
    accept_rate: i32,
}


#[derive(Debug)]
pub struct StackExchangeAnswer {
    pub owner: StackExchangeUser,
    pub is_accepted: bool,
    pub score: i32,
    pub answer_id: u64,
    pub text: String,
}


impl Decodable for StackExchangeAnswer {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.
    }
}
