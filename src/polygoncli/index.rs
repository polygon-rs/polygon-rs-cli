use crate::polygoncli::{daily::Daily, nbbo::NBBO};
use clap::Command;
use polygon_rs_api::security::{indices, Secuirty};

pub struct Index {}

impl Index {
    pub fn security() -> Secuirty {
        Secuirty::Indices(indices::Indices {})
    }
    pub fn command() -> Command {
        Command::new("index")
            .about("Use the Index API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('i')
            .subcommand(NBBO::command()
            )
            .subcommand(Daily::command()
            )
    }
}
