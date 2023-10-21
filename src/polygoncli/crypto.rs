use clap::{arg, Command};

pub struct Crypto {}

impl Crypto {
    pub fn command() -> Command {
        Command::new("crypto")
            .about("Use the Crypto API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('c')
            .subcommand(
                Command::new("nbbo")
                    .about("Call Crypto NNBO")
                    .arg_required_else_help(true),
            )
            .subcommand(
                Command::new("daily")
                    .about("Call Crypto Daily")
                    .arg_required_else_help(true),
            )
    }
}
