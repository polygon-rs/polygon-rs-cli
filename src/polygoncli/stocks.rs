use crate::polygoncli::{daily::Daily, nbbo::NBBO};
use clap::Command;
use polygon_rs_api::security::{stocks, Secuirty};

pub struct Stocks {}

impl Stocks {
    pub fn security() -> Secuirty {
        Secuirty::Stocks(stocks::Stocks {})
    }

    pub fn command() -> Command {
        Command::new("stocks")
            .about("Use the Stocks API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('s')
            .subcommand(NBBO::command())
            .subcommand(Daily::command())
    }
}
