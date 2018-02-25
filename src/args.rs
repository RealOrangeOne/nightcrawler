use clap::{App, AppSettings, ArgMatches, Arg};
use url::Url;

static VERBOSE_ARG_NAME: &str = "verbose";
static URL_ARG_NAME: &str = "url";

fn validate_url(input: String) -> Result<(), String> {
    let url_result = Url::parse(input.as_str());
    match url_result {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("{}", e)),
    };
}

fn build() -> App<'static, 'static> {
    return App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::StrictUtf8)
        .arg(Arg::with_name(VERBOSE_ARG_NAME).global(true).short("v").long("verbose").help(
            "Show verbose output"
        ))
        .arg(
            Arg::with_name(URL_ARG_NAME).help("URL to browse").required(true).validator(
                validate_url
            )
        );
}

pub fn get_matches() -> ArgMatches<'static> {
    return build().get_matches();
}

pub fn get_verbose(m: &ArgMatches) -> bool {
    return m.is_present(VERBOSE_ARG_NAME);
}

pub fn parse_url(m: &ArgMatches) -> Url {
    let url_value = match m.value_of(URL_ARG_NAME) {
        Some(url) => url,
        None => unreachable!(),  // required argument
    };
    return match Url::parse(url_value) {
        Ok(url) => url,
        Err(_) => unreachable!(),  // Validation is handled by arg parser
    };
}
