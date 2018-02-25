use clap::{App, AppSettings, ArgMatches, Arg};
use url::Url;

fn build() -> App<'static, 'static> {
    return App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::StrictUtf8)
        .arg(
            Arg::with_name("verbose")
                .global(true)
                .short("v")
                .long("verbose")
                .help("Show verbose output")
        )
        .arg(Arg::with_name("URL")
            .help("URL to browse")
            .required(true)
        );
}

pub fn get_matches() -> ArgMatches<'static> {
    return build().get_matches();
}

pub fn get_verbose(m: &ArgMatches) -> bool {
    return m.is_present("verbose");
}

pub fn parse_url(m: &ArgMatches) -> Url {
    return Url::parse(m.value_of("URL").unwrap()).unwrap();
}
