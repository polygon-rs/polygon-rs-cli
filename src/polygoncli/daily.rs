use clap::{arg, ArgMatches, Command};
use polygon_rs_api::{
    call::{daily, Call},
    polygon::{polygon::Polygon, sort::Sort, timespan::Timespan},
    security::Secuirty,
};

pub struct Daily {}

impl Daily {
    fn call() -> Option<Call> {
        Some(Call::Daily(daily::Daily {
            after_hours: None,
            close: None,
            from: None,
            high: None,
            low: None,
            open: None,
            pre_market: None,
            status: None,
            symbol: None,
            volume: None,
        }))
    }

    pub fn command() -> Command {
        Command::new("daily")
            .about("Call Daily")
            .arg_required_else_help(true)
            .arg(
                arg!(--ticker <Ticker> "Ticker symbol for Stock")
                    .num_args(1)
                    .default_value(None)
                    .default_missing_value(None)
                    .short('t'),
            )
    }

    pub fn request(args: &ArgMatches, security: Option<Secuirty>) {
        let p = Polygon::polygon(
            security,
            Self::call(),
            args.get_one::<String>("apikey").cloned(),
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
