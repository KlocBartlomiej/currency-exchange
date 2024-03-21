use crate::data::api_call_storage::ApiData;

pub fn print_available_currencies(api_data: ApiData) {
    println!("Available currencies: ");
    for currency in api_data.exchange_rates {
        println!("{} {}", currency.name, currency.full_name);
    }
}

pub fn calculate_exchange(api_data: ApiData, amount: &String, source: &String, target: &String) -> f32 {
    let mut source_rate = -1.0;
    let mut target_rate = -1.0;
    for exchange in api_data.exchange_rates {
        if source.eq(&exchange.name) {
            source_rate = exchange.rate;
        }
        if target.eq(&exchange.name) {
            target_rate = exchange.rate;
        }
    }
    if source_rate < 0.0 {
        //error source currency not found
    }
    if target_rate < 0.0 {
        //error target currency not found
    }
    let rate = target_rate / source_rate;
    let new_amount = amount.parse::<f32>().unwrap() * rate; //TODO check if parsable
    println!("Exchanged {} from {} is {:.2} {}, rate = {}", amount, source, new_amount, target, rate);
    new_amount
}