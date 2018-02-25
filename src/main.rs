#[macro_use]
extern crate clap;
extern crate url;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate webkit2gtk;

mod args;
mod config;

fn main() {
    config::from_args(args::get_matches());
}
