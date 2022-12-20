use std::env;

mod currency_api;

fn main() {
    println!("First Rust Project...");
    let args: Vec<String> = env::args().collect();
    let from_currency = args[1].to_string();
    let to_currency = args[2].to_string();
    let amount = args[3].parse::<f32>().expect("wrong input");

    currency_api::client::convert_currency(from_currency, to_currency, amount);
}