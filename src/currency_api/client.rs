use crate::currency_api::response::CurrencyResponse;

pub fn convert_currency (from: String, to: String, amount:f32) {

    let url = format!("https://api.exchangerate.host/convert?from={}&to={}&amount={}", from, to, amount);
    println!("The url {}", url);

    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.json::<CurrencyResponse>() {
                    Ok(resp) =>
                        println!("Coversion Rate is {} and the coverted amount is {}", resp.info.rate, resp.result),
                    Err(_) => println!("Could not read text")
                }
            }
        }
        Err(_) => println!("Could not make request")
    }
}