#![cfg(test)]

use std::path::Path;
use std::fs;

use crate::data::api_call_storage::{FILE_PATH, RATE_FILE_PATH, CURRENCY_FILE_PATH, ApiData, ExchangeRate};
use crate::data::populate_data::get_api_data;

fn create_cache_for_test() {
    let currencies = r#"
    {
        "EUR": "Euro",
        "PLN": "Polish Złoty",
        "USD": "United States Dollar"
    }
    "#;
    let rates = r#"
    {
        "amount": 1.0,
        "base": "EUR",
        "date": "2024-03-15",
        "rates": {
            "PLN": 4.0,
            "USD": 1.1
        }
    }
    "#;
    let file_path = &FILE_PATH.to_string();
        let my_path = Path::new::<String>(&file_path);
        if !my_path.exists() {
            _ = fs::create_dir(FILE_PATH);
        }
        fs::write(RATE_FILE_PATH, rates).expect("Unable to save rate data");
        fs::write(CURRENCY_FILE_PATH, currencies).expect("Unable to save currency data");
}

fn create_api_data_for_test() -> ApiData {
    let mut api_data = ApiData{
        exchange_rates: Vec::<ExchangeRate>::with_capacity(3)
    };

    api_data.exchange_rates.push(
        ExchangeRate{
            name: "EUR".to_string(),
            full_name: "\"Euro\"".to_string(),
            rate: 1.0
        }
    );

    api_data.exchange_rates.push(
        ExchangeRate{
            name: "PLN".to_string(),
            full_name: "\"Polish Złoty\"".to_string(),
            rate: 4.0
        }
    );

    api_data.exchange_rates.push(
        ExchangeRate{
            name: "USD".to_string(),
            full_name: "\"United States Dollar\"".to_string(),
            rate: 1.1
        }
    );

    api_data.exchange_rates.sort_by(|a, b| b.name.cmp(&a.name));
    api_data
}

#[test]
fn test_create_api_data_and_validate() {
    create_cache_for_test();
    let mut result = get_api_data();
    result.exchange_rates.sort_by(|a, b| b.name.cmp(&a.name));
    assert_eq!(result, create_api_data_for_test());
    _ = fs::remove_dir_all(FILE_PATH);
}