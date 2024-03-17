mod subcommands;

use subcommands::select_proper_mod;

#[tokio::main(flavor = "current_thread")]
async fn main(){
    select_proper_mod();
}
