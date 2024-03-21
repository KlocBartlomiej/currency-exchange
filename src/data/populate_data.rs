use crate::data::api_call_storage::{ApiData, ApiStorage, StorageCommand};
use crate::data::api_call_parser::parse_and_return_data;

pub fn get_api_data() -> ApiData {
    let mut api_storage = ApiStorage::new();
    let mut response: (String, String) = (String::new(), String::new());
    if !api_storage.is_data_up_to_date_and_valid {
        response = get_api_response();
        api_storage.execute(StorageCommand::Store(response.clone().0, response.clone().1));
    }
    match api_storage.execute(StorageCommand::Get) {
        Some(result) => result,
        None => {
            if response.0.is_empty() {
                response = get_api_response();
            }
            match parse_and_return_data(response.0.as_str(), response.1.as_str()) {
                Some(result) => result,
                None => {
                    panic!("Can't retrieve data.");
                },
            }
        },
    }
}

fn get_api_response() -> (String, String) {
    let rate = match reqwest::blocking::get("https://api.frankfurter.app/latest") {
        Ok(rate) => rate.text(),
        Err(error) => panic!("Cannot acquire data from external api: {:?}", error),
    };
    let currency = match reqwest::blocking::get("https://api.frankfurter.app/currencies") {
        Ok(currency) => currency.text(),
        Err(error) => panic!("Cannot acquire data from external api: {:?}", error),
    };
    (rate.unwrap(), currency.unwrap())
}