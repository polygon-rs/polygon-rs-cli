use crate::polygoncli::PolygonCLI;
use clap::{arg, ArgMatches, Command};
use polygon_rs_api::{
    call::{nbbo, Call},
    polygon::{polygon::Polygon, sort::Sort, timespan::Timespan},
    security::Secuirty,
};

pub struct NBBO {}

impl NBBO {
    fn call() -> Option<Call> {
        Some(Call::NBBO(nbbo::NBBO {
            next_url: None,
            request_id: None,
            results: None,
            status: None,
        }))
    }

    pub fn command() -> Command {
        Command::new("nbbo")
            .about("Call NNBO")
            .arg_required_else_help(true)
            .arg(
                arg!(--ticker <Ticker> "Ticker symbol for Stock")
                    .num_args(1)
                    .default_value("SPY")
                    .default_missing_value("SPY")
                    .short('t'),
            )
            .arg(
                arg!(--key <APIKEY> "API Key for interacting with polygon.io")
                    .num_args(1)
                    .default_value(None)
                    .default_missing_value(None)
                    .short('k'),
            )
    }

    pub fn request(args: &ArgMatches, security: Option<Secuirty>) {
        let p = Polygon::polygon(
            security,
            Self::call(),
            if args.get_one::<String>("apikey").cloned() == None {Some(PolygonCLI::checkforapikey())} else {args.get_one::<String>("apikey").cloned()},
            args.get_one::<String>("ticker").cloned(),
            args.get_one::<u16>("multiplier").cloned(),
            args.get_one::<Timespan>("timespan").cloned(),
            args.get_one::<String>("from").cloned(),
            args.get_one::<String>("to").cloned(),
            args.get_one::<bool>("adjusted").cloned(),
            args.get_one::<Sort>("sort").cloned(),
            args.get_one::<u16>("limit").cloned(),
            None,
        );
        match p.request() {
            Ok(r) => println!("{:?}", r),
            Err(e) => panic!("{}", e),
        }
    }
}
