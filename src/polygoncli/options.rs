use crate::polygoncli::{daily::Daily, nbbo::NBBO};
use clap::Command;
use polygon_rs_api::security::{options, Secuirty};
pub struct Options {}

impl Options {
    pub fn security() -> Secuirty {
        Secuirty::Options(options::Options {})
    }

    pub fn command() -> Command {
        Command::new("options")
            .about("Use the Options API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('o')
            .subcommand(NBBO::command())
            .subcommand(Daily::command()
            )
    }
}
