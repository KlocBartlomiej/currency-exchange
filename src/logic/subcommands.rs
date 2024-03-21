use clap::{command, Command, Arg};

use crate::data::api_call_storage::ApiData;
use crate::logic::interactive::run_interactive_mode;
use crate::logic::calculation::{print_available_currencies, calculate_exchange};

pub fn select_and_execute_proper_mod(api_data: ApiData) {
    let match_result = command!()
    .subcommand(
        Command::new("interactive")
        .about("Enters interactive mod for calculating conversions")
    )
    .subcommand(
        Command::new("exchange")
        .about("This command returns amount after conversion with it's current exchange rate")
        .arg(
            Arg::new("source-currency")
                .required(true)
                .short('s')
                .long("source-currency")
                .help("From this currency amount will be converted from")
        )
        .arg(
            Arg::new("target-currency")
                .required(true)
                .short('t')
                .long("target-currency")
                .help("This is the currency amount will be converter to")
        )
        .arg(
            Arg::new("amount")
                .required(true)
                .short('a')
                .long("amount")
                .help("This is the amount of money application will convert")
        )
    )
    .subcommand(
        Command::new("list")
            .about("This option will print out all available currencies you can convert from and to")
    ).get_matches();

    match match_result.subcommand(){
        Some(("interactive", _)) => {
            run_interactive_mode(api_data);
        },
        Some(("exchange", sub_m)) => {
            let source = sub_m.get_one::<String>("source-currency").unwrap();
            let target = sub_m.get_one::<String>("target-currency").unwrap();
            let amount = sub_m.get_one::<String>("amount").unwrap();
            calculate_exchange(&api_data,amount,source,target);
        },
        Some(("list", _)) => {
            print_available_currencies(api_data);
        },
        _ => {
            println!("Please use --help to see all available options.");
        }
    }
}