use crate::populate_data::ApiData;

pub fn print_available_currencies(api_data: ApiData) {
    println!("Available currencies: ");
    for exchange in api_data.exchange_rates {
        println!("{} {}", exchange.name, exchange.full_name);
    }
}

pub fn calculate_exchange(api_data: ApiData, amount: &String, source: &String, target: &String) {
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
    } else if target_rate < 0.0 {
        //error target currency not found
    } else {
        let rate = target_rate / source_rate;
        let new_amount = amount.parse::<f32>().unwrap() * rate; //TODO check if parsable
        println!("Exchanged {} from {} is {} {}, rate = {}", amount, source, new_amount, target, rate);
    }
}