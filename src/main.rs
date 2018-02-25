#[macro_use]
extern crate clap;
extern crate url;

mod args;
mod config;


fn main() {
    config::from_args(args::get_matches());
}
