use bac_journal::person::Gender;
use bac_journal::{Drink, Person, BAC};
extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

type Error = std::io::Error;

fn get_matches() -> ArgMatches<'static> {
    App::new("Alcohol Mate")
        .version("1.0")
        .author("Aaron B. <burdick.aaron@gmail.com>")
        .about("Helps you drink alcohol at a pace that is healthy")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add a drink")
                .arg(
                    Arg::with_name("drink")
                        .short("d")
                        .required(true)
                        .help("The name of the drink"),
                )
                .arg(
                    Arg::with_name("percent")
                        .short("p")
                        .required(true)
                        .help("The percentage of the drink"),
                )
                .arg(
                    Arg::with_name("weight")
                        .short("w")
                        .required(true)
                        .help("The weight of the drink"),
                ),
        )
        .get_matches()
}

fn cli_loop() {
    loop {}
}

fn with_config(config: &str, matches: &ArgMatches<'static>) -> Result<(), Error> {
    println!();
    Ok(())
}

fn without_config(matches: &ArgMatches<'static>) -> Result<(), Error> {
    Ok(())
}

fn main() -> Result<(), Error> {
    let matches = get_matches();

    match &matches.value_of("config") {
        Some(config) => with_config(config, &matches)?,
        None => without_config(&matches)?,
    };

    Ok(())
}
