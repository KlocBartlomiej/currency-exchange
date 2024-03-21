#![cfg(test)]

use crate::data::api_call_storage::{ApiData, ExchangeRate};
use crate::logic::calculation::{print_available_currencies, calculate_exchange};

fn prepare_api_data_for_test() -> ApiData {
    let mut api_data = ApiData{
        exchange_rates: Vec::<ExchangeRate>::with_capacity(2)
    };

    api_data.exchange_rates.push(
        ExchangeRate{
            name: "PLN".to_string(),
            full_name: "Polski złoty".to_string(),
            rate: 1.0
        }
    );

    api_data.exchange_rates.push(
        ExchangeRate{
            name: "USD".to_string(),
            full_name: "Dollar amerykański".to_string(),
            rate: 0.25
        }
    );

    api_data
}

#[test]
fn test_print_functionality(){
   print_available_currencies(prepare_api_data_for_test());
}

#[test]
fn test_calculation(){
    let result = calculate_exchange(&prepare_api_data_for_test(), &"100.0".to_string(), &"PLN".to_string(), &"USD".to_string());
    assert_eq!(result, Some(25.0));
}

#[test]
fn test_calculation_target_not_found(){
    let result = calculate_exchange(&prepare_api_data_for_test(), &"100.0".to_string(), &"Polski złoty".to_string(), &"USD".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_calculation_source_not_found(){
    let result = calculate_exchange(&prepare_api_data_for_test(), &"100.0".to_string(), &"PLN".to_string(), &"Dollar amerykański".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_calculation_invalid_amount(){
    let result = calculate_exchange(&prepare_api_data_for_test(), &"non_parsable_to_float_string".to_string(), &"PLN".to_string(), &"USD".to_string());
    assert_eq!(result, None);
}