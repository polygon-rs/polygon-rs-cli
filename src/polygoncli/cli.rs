use crate::polygoncli::{
    crypto::Crypto, forex::Forex, index::Index, options::Options, stocks::Stocks,
};
use clap::Command;
pub struct CLI {}

impl CLI {
    pub fn cli() -> Command {
        Command::new("polygon")
            .about("A CLI for interacting with the polygon.io api.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .allow_external_subcommands(true)
            .subcommand(Stocks::command())
            .subcommand(Options::command())
            .subcommand(Index::command())
            .subcommand(Forex::command())
            .subcommand(Crypto::command())
    }
}
