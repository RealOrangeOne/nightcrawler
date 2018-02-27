#[macro_use]
extern crate clap;
extern crate gtk;
extern crate gdk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate url;

extern crate webkit2gtk;

use relm::Widget;

mod args;
mod config;
mod window;
mod webview;

fn main() {
    let app_config = config::from_args(args::get_matches());
    window::Win::run(app_config).unwrap();
}
