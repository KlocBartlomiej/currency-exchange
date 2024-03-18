use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::de::from_str;
use serde_json::Value;


#[derive(Deserialize, Debug)]
struct Response {
    amount: f32,
    base: String,
    date: String,
    rates: HashMap<String,f32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiData {
    pub date: String,
    pub exchange_rates: Vec<ExchangeRate>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExchangeRate {
    pub name : String,
    pub full_name : String,
    pub rate : f32,
}

pub fn get_api_data() -> ApiData {    //TODO it should check if value update is needed
    // let response = reqwest::blocking::get("https://api.frankfurter.app/latest").unwrap().text();
    let response = r#"
    {
        "amount": 1.0,
        "base": "EUR",
        "date": "2024-03-15",
        "rates": {
            "AUD": 1.6579,
            "BGN": 1.9558,
            "BRL": 5.4461,
            "CAD": 1.4731,
            "CHF": 0.9613,
            "CNY": 7.838,
            "CZK": 25.166,
            "DKK": 7.4571,
            "GBP": 0.8541,
            "HKD": 8.5199,
            "HUF": 393.2,
            "IDR": 17011,
            "ILS": 3.9811,
            "INR": 90.26,
            "ISK": 148.9,
            "JPY": 162.03,
            "KRW": 1448.71,
            "MXN": 18.1915,
            "MYR": 5.1241,
            "NOK": 11.5205,
            "NZD": 1.786,
            "PHP": 60.494,
            "PLN": 4.2953,
            "RON": 4.9711,
            "SEK": 11.2674,
            "SGD": 1.4562,
            "THB": 39.053,
            "TRY": 35.092,
            "USD": 1.0892,
            "ZAR": 20.352
        }
    }
    "#;

    // let response2 = reqwest::blocking::get("https://api.frankfurter.app/currencies").unwrap().text();
    let response2 = r#"
    {
        "AUD": "Australian Dollar",
        "BGN": "Bulgarian Lev",
        "BRL": "Brazilian Real",
        "CAD": "Canadian Dollar",
        "CHF": "Swiss Franc",
        "CNY": "Chinese Renminbi Yuan",
        "CZK": "Czech Koruna",
        "DKK": "Danish Krone",
        "EUR": "Euro",
        "GBP": "British Pound",
        "HKD": "Hong Kong Dollar",
        "HUF": "Hungarian Forint",
        "IDR": "Indonesian Rupiah",
        "ILS": "Israeli New Sheqel",
        "INR": "Indian Rupee",
        "ISK": "Icelandic Króna",
        "JPY": "Japanese Yen",
        "KRW": "South Korean Won",
        "MXN": "Mexican Peso",
        "MYR": "Malaysian Ringgit",
        "NOK": "Norwegian Krone",
        "NZD": "New Zealand Dollar",
        "PHP": "Philippine Peso",
        "PLN": "Polish Złoty",
        "RON": "Romanian Leu",
        "SEK": "Swedish Krona",
        "SGD": "Singapore Dollar",
        "THB": "Thai Baht",
        "TRY": "Turkish Lira",
        "USD": "United States Dollar",
        "ZAR": "South African Rand"
    }
    "#;

    // let result = from_str::<Response>(response.unwrap().as_str());
    let result = from_str::<Response>(response);
    if result.is_err() {
        println!("{:#?}", result.as_ref().err())
    }
    let currency = result.unwrap();

    
    // let v = from_str::<Value>(response2.unwrap().as_str());
    let v = from_str::<Value>(response2);
    if v.is_err() {
        println!("{:#?}", v.as_ref().err())
    }

    let val = v.unwrap();
    create_and_return_date(currency,val)
}

fn create_and_return_date(currency : Response, v : Value) -> ApiData {
    let mut api_data = ApiData{
        date: currency.date,
        exchange_rates: Vec::<ExchangeRate>::with_capacity(currency.rates.capacity()+1)
    };

    api_data.exchange_rates.push(
        ExchangeRate{
            name: currency.base,
            full_name: v["EUR"].to_string(),
            rate: currency.amount
        }
    );

    for(key,value) in currency.rates{
        api_data.exchange_rates.push(
            ExchangeRate{
                name: key.clone(),
                full_name: v[key].to_string(),
                rate: value
            }
        );
    }
    api_data
}