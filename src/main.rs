mod data;
mod logic;

mod test;

use data::populate_data::get_api_data;
use logic::subcommands::select_and_execute_proper_mod;

fn main(){
    let api_data = get_api_data();
    select_and_execute_proper_mod(api_data);
}

