use std::collections::HashMap;
use serde::Deserialize;
use serde_json::de::from_str;
use serde_json::Value;

use crate::data::api_call_storage::{ApiData, ExchangeRate};

#[derive(Deserialize, Debug)]
pub struct Response {
    amount: f32,
    base: String,
    rates: HashMap<String,f32>,
}

pub fn parse_and_return_data(rate : &str, currency : &str) -> Option<ApiData> {

    let rate_parse = match from_str::<Response>(rate) {
        Ok(rate_parse) => rate_parse,
        Err(error) => {
            println!("Could not parse rates. {}", error);
            return None;
        }
    };
    let currency_value = match from_str::<Value>(currency) {
        Ok(currency_value) => currency_value,
        Err(error) => {
            println!("Could not parse currencies. {}", error);
            return None;
        }
    };

    let mut api_data = ApiData{
        exchange_rates: Vec::<ExchangeRate>::with_capacity(rate_parse.rates.capacity()+1)
    };

    api_data.exchange_rates.push(
        ExchangeRate{
            name: rate_parse.base.clone(),
            full_name: currency_value[rate_parse.base].to_string(),
            rate: rate_parse.amount
        }
    );

    for(key,value) in rate_parse.rates{
        api_data.exchange_rates.push(
            ExchangeRate{
                name: key.clone(),
                full_name: currency_value[key].to_string(),
                rate: value
            }
        );
    }
    Some(api_data)
}