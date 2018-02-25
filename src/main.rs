#[macro_use]
extern crate clap;
extern crate url;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate webkit2gtk;

use relm::Widget;

mod args;
mod config;
mod window;

fn main() {
    let app_config = config::from_args(args::get_matches());
    window::Win::run(app_config).unwrap();
}
