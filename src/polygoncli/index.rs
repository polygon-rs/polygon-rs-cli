use clap::{arg, Command};

pub struct Index {}

impl Index {
    pub fn command() -> Command {
        Command::new("index")
            .about("Use the Index API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('i')
            .subcommand(
                Command::new("nbbo")
                    .about("Call Index NNBO")
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("daily")
                    .about("Call Index Daily")
                    .arg_required_else_help(true),
            )
    }
}
