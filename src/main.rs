mod populate_data;
mod subcommands;
mod interactive;
mod calculation;

use populate_data::get_api_data;
use subcommands::select_proper_mod;

fn main(){
    let api_data = get_api_data();
    select_proper_mod(api_data);
}

