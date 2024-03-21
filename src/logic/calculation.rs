use crate::data::api_call_storage::ApiData;

pub fn print_available_currencies(api_data: ApiData) {
    println!("Available currencies: ");
    for currency in api_data.exchange_rates {
        println!("{} {}", currency.name, currency.full_name);
    }
}

pub fn calculate_exchange(api_data: &ApiData, amount: &String, source: &String, target: &String) -> Option<f32> {
    let mut source_rate = f32::NAN;
    let mut target_rate = f32::NAN;
    for exchange in &api_data.exchange_rates {
        if source.eq(&exchange.name) {
            source_rate = exchange.rate;
        }
        if target.eq(&exchange.name) {
            target_rate = exchange.rate;
        }
    }
    if source_rate.is_nan() {
        println!("Source currency not found, please provide valid source currency. You can use list subcommand to print available currencies.");
        return None;
    }
    if target_rate.is_nan() {
        println!("Target currency not found, please provide valid target currency. You can use list subcommand to print available currencies.");
        return None;
    }
    let rate = target_rate / source_rate;
    let numeric_amount = match amount.parse::<f32>() {
        Ok(number) => number,
        Err(error) => {
            println!("Provided amount is not valid, please make sure you provided a valid number (eg. no whitespaces allowed). {}", error);
            return None;
        },
    };
    let new_amount = numeric_amount * rate;
    println!("Exchanged {} from {} is {:.2} {}, rate = {}", amount, source, new_amount, target, rate);
    Some(new_amount)
}