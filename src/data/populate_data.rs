use crate::data::api_call_storage::{ApiData, ApiStorage, StorageCommand};

pub fn get_api_data() -> ApiData {
    let mut api_storage = ApiStorage::new();
    if !api_storage.is_data_up_to_date {
        let response = get_api_response();
        api_storage.execute(StorageCommand::Store(response.0, response.1));
    }
    api_storage.execute(StorageCommand::Get).unwrap()
}

fn get_api_response() -> (String, String) {
    let rate = reqwest::blocking::get("https://api.frankfurter.app/latest").unwrap().text();
    let currency = reqwest::blocking::get("https://api.frankfurter.app/currencies").unwrap().text();
    (rate.unwrap(), currency.unwrap())
}