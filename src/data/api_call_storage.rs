use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs::{self, File};

use crate::data::api_call_parser::parse_and_return_data;

pub const FILE_PATH: &str = "./target/data";
pub const CURRENCY_FILE_PATH: &str = "./target/data/currency.json";
pub const RATE_FILE_PATH: &str = "./target/data/rate.json";
const REFRESH_DATA_OFFSET : u64 = 10*60;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ApiData {
    pub exchange_rates: Vec<ExchangeRate>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ExchangeRate {
    pub name : String,
    pub full_name : String,
    pub rate : f32,
}

pub enum StorageCommand<> {
    Store(String, String),
    Get,
}

pub struct ApiStorage {
    pub is_data_up_to_date_and_valid: bool,
}

impl ApiStorage {
    pub fn new() -> ApiStorage {
        let file_path = &FILE_PATH.to_string();
        let my_path = Path::new::<String>(&file_path);
        if !my_path.exists() {
            fs::create_dir(FILE_PATH).expect("Error  occured during creation of data dir");
        }
        if !Path::new(CURRENCY_FILE_PATH).exists() ||
            !Path::new(RATE_FILE_PATH).exists(){
                return ApiStorage { is_data_up_to_date_and_valid: false };
        }
        let file = File::open(CURRENCY_FILE_PATH).expect("Error occured during data accessing");
        let metadata = file.metadata().unwrap();
        let time_modified = metadata.modified().unwrap();
        if time_modified.elapsed().unwrap().as_secs() > REFRESH_DATA_OFFSET {
            return ApiStorage { is_data_up_to_date_and_valid: false };
        }
        ApiStorage { is_data_up_to_date_and_valid: true }
    }

    pub fn execute(&mut self, command: StorageCommand) -> Option<ApiData> {
        match command {
            StorageCommand::Store(rates, currencies) => {
                fs::write(RATE_FILE_PATH, rates).expect("Unable to save rate data");
                fs::write(CURRENCY_FILE_PATH, currencies).expect("Unable to save currency data");
                None
            },
            StorageCommand::Get => {
                let rate: String = fs::read_to_string(RATE_FILE_PATH).expect("Unable to read rate data");
                let currency: String = fs::read_to_string(CURRENCY_FILE_PATH).expect("Unable to read currency data");
                parse_and_return_data(rate.as_str(), currency.as_str())
            }
        }
    }
}