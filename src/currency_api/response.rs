use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CurrencyResponse {
    pub motd: Motd,
    pub success: bool,
    pub query: Query,
    pub info: Info,
    pub historical: bool,
    pub date: String,
    pub result: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Motd {
    msg: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    from: String,
    to: String,
    amount: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub rate: f32
}
