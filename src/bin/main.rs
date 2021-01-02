use bac_journal::{User};
extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env::current_dir;
use std::path::{PathBuf};

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

fn cli_loop(user: User) {
    let mut buf = String::new();
    loop {
        println!("{}", &user.bac.as_float());
        std::io::stdin().read_line(&mut buf);
        if !buf.is_empty() {
            break;
        }
    }
}

fn main() -> Result<(), Error> {
    let matches = get_matches();

    let dir_path = match &matches.value_of("config") {
        Some(config_path) => PathBuf::from(config_path),
        None => {
            let dir_path = current_dir()?;
            println!("No config path provided. Using {}", dir_path.to_str().unwrap());
            dir_path
        },
    };

    let user = User::open(dir_path)?;
    cli_loop(user);

    Ok(())
}
