use clap::ArgMatches;
use url::Url;

use args::{get_verbose, parse_url};

pub struct Config {
    verbose: bool,
    initial_url: Url
}


pub fn from_args(args: ArgMatches) -> Config {
    let config = Config {
        verbose: get_verbose(&args),
        initial_url: parse_url(&args)
    };
    return config;
}
