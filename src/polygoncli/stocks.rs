use crate::polygoncli::nbbo::NBBO;
use clap::{arg, ArgMatches, Command};
use polygon_rs_api::security::{stocks, Secuirty};

pub struct Stocks {}

impl Stocks {
    fn security() -> Secuirty {
        Secuirty::Stocks(stocks::Stocks {})
    }

    pub fn command() -> Command {
        Command::new("stocks")
            .about("Use the Stocks API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('s')
            .subcommand(
                Command::new("nbbo")
                    .about("Call Stock NNBO")
                    .arg_required_else_help(true)
                    .arg(
                        arg!(--ticker <Ticker> "Ticker symbol for Stock")
                            .num_args(1)
                            .default_value(None)
                            .default_missing_value(None)
                            .short('t'),
                    ),
            )
            .subcommand(
                Command::new("daily")
                    .about("Call Stock Daily")
                    .arg_required_else_help(true),
            )
    }

    pub fn parse(commands: &ArgMatches) {
        match commands.subcommand() {
            Some(("nbbo", sub_matches)) => {
                NBBO::request(sub_matches, Some(Self::security()));
            }
            _ => unreachable!(),
        }
    }
}
