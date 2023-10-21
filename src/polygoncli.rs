pub mod cli;
pub mod crypto;
pub mod forex;
pub mod index;
pub mod options;
pub mod stocks;
pub mod nbbo;
use crate::polygoncli::{cli::CLI, stocks::Stocks};

pub struct PolygonCLI {}

impl PolygonCLI {
    pub fn polygoncli() {
        let matches = CLI::cli().get_matches();
        match matches.subcommand() {
            Some(("stocks", stocks_command)) => 
                Stocks::parse(stocks_command),
            Some(("options", sub_matches)) => {
                println!(
                    "Cloning {}",
                    sub_matches.get_one::<String>("REMOTE").expect("required")
                );
            }
            Some(("indices", sub_matches)) => {
                println!(
                    "Cloning {}",
                    sub_matches.get_one::<String>("REMOTE").expect("required")
                );
            }
            Some(("forex", sub_matches)) => {
                println!(
                    "Cloning {}",
                    sub_matches.get_one::<String>("REMOTE").expect("required")
                );
            }
            Some(("crypto", sub_matches)) => {
                println!(
                    "Cloning {}",
                    sub_matches.get_one::<String>("REMOTE").expect("required")
                );
            }

            _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
        }
    }
}
