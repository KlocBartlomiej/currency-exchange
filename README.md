# Currency Exchange

Using [Frankfurter API](https://www.frankfurter.app/)

## How to build

After cloning repo you can build it with cargo using `cargo build`.
You can use now currency-exchange (./target/debug/currency-exchange).

## How to use

By default running application only print information about flag --help.
Using it will reveal all available commands:
- `interactive` which enter mode in which user is prompted to enter data
- `exchange` the default mode
- `list` which prints all available currencies

Default mode is exchange subcommand which requires options:
- `--amount` (-a) -> amount of money to conversion
- `--source-currency` (-s) -> currency from which amount will be converted
- `--target-currency` (-t) -> target currency for amount to be converted to

## Examples

Right after building app you can use it from project dir.

`./target/debug/currency-exchange exchange -a 150 -s EUR -t PLN`
