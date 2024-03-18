use crate::populate_data::ApiData;
use crate::calculation::calculate_exchange;

pub fn run_interactive_mode(api_data: ApiData) {    //TODO implement this function / remove placeholder
    use std::io::{stdin,stdout,Write};
    println!("Provide amount you'd like to exchange as well as target and source currency:");
    let mut amount = String::new();
    let mut source = String::new();
    let mut target = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut amount).expect("Did not enter a correct string");
    stdin().read_line(&mut source).expect("Did not enter a correct string");
    stdin().read_line(&mut target).expect("Did not enter a correct string");
    calculate_exchange(api_data,&amount,&source,&target);
}