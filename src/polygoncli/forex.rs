use clap::{arg, Command};

pub struct Forex {}

impl Forex {
    pub fn command() -> Command {
        Command::new("forex")
            .about("Use the Forex API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('f')
            .subcommand(
                Command::new("nbbo")
                    .about("Call Forex NNBO")
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("daily")
                    .about("Call Forex Daily")
                    .arg_required_else_help(true),
            )
    }
}
