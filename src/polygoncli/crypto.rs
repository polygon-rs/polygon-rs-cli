use crate::polygoncli::{daily::Daily, nbbo::NBBO};
use clap:: Command;
use polygon_rs_api::security::{crypto, Secuirty};

pub struct Crypto {}

impl Crypto {
    pub fn security() -> Secuirty {
        Secuirty::Crypto(crypto::Crypto {})
    }
    pub fn command() -> Command {
        Command::new("crypto")
            .about("Use the Crypto API's")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .short_flag('c')
            .subcommand(
                NBBO::command()
            )
            .subcommand(
                Daily::command()
            )
    }
}
