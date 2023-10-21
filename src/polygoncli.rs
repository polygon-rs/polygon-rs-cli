pub mod cli;
pub mod crypto;
pub mod forex;
pub mod index;
pub mod options;
pub mod stocks;
pub mod nbbo;
pub mod daily;
use clap::ArgMatches;
use crate::polygoncli::{cli::CLI, stocks::Stocks, options::Options, index::Index, forex::Forex, crypto::Crypto, nbbo::NBBO, daily::Daily};
use polygon_rs_api::security::Secuirty;

pub struct PolygonCLI {}

impl PolygonCLI {
    fn parse(commands: &ArgMatches, security: Secuirty) {
        match commands.subcommand() {
            Some(("nbbo", sub_matches)) => {
                NBBO::request(sub_matches, Some(security));
            },
            Some(("daily", sub_matches)) => {
                Daily::request(sub_matches, Some(security));
            },
            _ => unreachable!(),
        }
    }

    pub fn polygoncli() {
        let matches = CLI::cli().get_matches();
        match matches.subcommand() {
            Some(("stocks", stocks_command)) => 
                Self::parse(stocks_command, Stocks::security()),
            Some(("options", options_command)) => 
                Self::parse(options_command, Options::security()),
            Some(("indices", index_command)) => 
                Self::parse(index_command, Index::security()),
            Some(("forex", forex_command)) => 
                Self::parse(forex_command, Forex::security()),
            Some(("crypto", crypto_command)) => 
                Self::parse(crypto_command, Crypto::security()),
            _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
        }
    }
}
