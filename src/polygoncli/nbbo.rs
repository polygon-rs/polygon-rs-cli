use clap::ArgMatches;
use polygon_rs_api::{
    call::{nbbo, Call},
    polygon::{polygon::Polygon, timespan::Timespan, sort::Sort},
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
