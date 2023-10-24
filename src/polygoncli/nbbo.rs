use crate::polygoncli::PolygonCLI;
use clap::{arg, ArgMatches, Command, value_parser};
use polygon_rs_api::{
    call::{nbbo, Call},
    polygon::{polygon::Polygon, sort::Sort},
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
                arg!(--ticker <Ticker> "The ticker symbol to get quotes for.")
                    .num_args(1)
                    .default_value("SPY")
                    .default_missing_value("SPY")
                    .short('t')
                    .required(true),
            )
            .arg(
                arg!(--key <ApiKey> "API Key for interacting with polygon.io")
                    .num_args(1)
                    .default_value(None)
                    .default_missing_value(None)
                    .short('k'),
            )
            .arg(
                arg!(--timestamp <Timestamp> "Query by timestamp. Either a date with the format YYYY-MM-DD or a nanosecond timestamp.")
                    .num_args(1)
                    .default_value(None)
                    .default_missing_value(None)
                    .short('T'),
            )
            .arg(
                arg!(--order <Order> "Order results based on the sort field.")
                    .num_args(1)
                    .default_value(None)
                    .default_missing_value("timestamp")
                    .short('o')
                    .value_parser(["timestamp"]),
            )
            .arg(
                arg!(--limit <Limit> "Limit the number of results returned, default is 10 and max is 50000.")
                    .num_args(1)
                    .default_value(None)
                    .default_missing_value(None)
                    .short('l')
                    .value_parser(value_parser!(u16)),
            )
            .arg(
                arg!(--sort <Sort> "Sort field used for ordering.")
                    .num_args(1)
                    .default_value(None)
                    .default_missing_value(None)
                    .value_parser(["asc", "desc"])
                    .short('s'),
            )
    }

    pub fn request(args: &ArgMatches, security: Option<Secuirty>) {
        let p = Polygon::polygon(
            security,
            Self::call(),
            if args.get_one::<String>("apikey").cloned() == None {Some(PolygonCLI::checkforapikey())} else {args.get_one::<String>("apikey").cloned()},
            args.get_one::<String>("ticker").cloned(),
            None,
            None,
            args.get_one::<String>("from").cloned(),
            args.get_one::<String>("to").cloned(),
            args.get_one::<bool>("adjusted").cloned(),
            args.get_one::<Sort>("sort").cloned(),
            args.get_one::<u16>("limit").cloned(),
            args.get_one::<String>("timestamp").cloned(),
        );
        match p.request() {
            Ok(r) => println!("{:?}", r),
            Err(e) => panic!("{}", e),
        }
    }
}
