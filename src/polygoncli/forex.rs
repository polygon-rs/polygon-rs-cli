use crate::polygoncli::{daily::Daily, nbbo::NBBO};
use clap::Command;
use polygon_rs_api::security::{forex, Secuirty};

pub struct Forex {}

impl Forex {
    pub fn security() -> Secuirty {
        Secuirty::Forex(forex::Forex {})
    }
    pub fn command() -> Command {
        Command::new("forex")
            .about("Use the Forex API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('f')
            .subcommand(NBBO::command()
            )
            .subcommand(Daily::command()
            )
    }
}
