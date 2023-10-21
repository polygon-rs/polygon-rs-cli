use clap::{arg, Command};

pub struct Options {}

impl Options {
    pub fn command() -> Command {
        Command::new("options")
            .about("Use the Options API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('o')
            .subcommand(
                Command::new("nbbo")
                    .about("Call Option NNBO")
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("daily")
                    .about("Call Option Daily")
                    .arg_required_else_help(true),
            )
    }
}
