use clap::ArgMatches;
use url::Url;

use args::{get_verbose, parse_url};

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: bool,
    initial_url: Url,
}

impl Config {
    pub fn get_initial_url(&self) -> &str {
        return self.initial_url.as_str();
    }
}

pub fn from_args(args: ArgMatches) -> Config {
    let config = Config {
        verbose: get_verbose(&args),
        initial_url: parse_url(&args),
    };
    return config;
}
