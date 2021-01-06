use bac_journal::drink::legal_limits;
use bac_journal::{EffectInfo, User};
extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::env::current_dir;
use std::path::PathBuf;

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

fn drink_report(user: &User) {
    let drink_len = user.bac.drink_len();
    let msg = match drink_len {
        len if drink_len > 1 => format!("You've had {} drinks", len),
        1 => "You've had 1 drink".to_string(),
        0 => "You've had no drinks".to_string(),
        _ => panic!("Invalid drink num"),
    };
    println!("{}", msg);
    println!("");

    user.bac.report_first_drink();
    for d in user.bac.drinks.iter() {
        println!("{}", d.report());
        println!("");
    }
}

fn report_legal_limit(user: &User) {
    let limits = legal_limits();
    // TODO: Implement way of following user's country
    let country = "Japan".to_string();
    if let Some(limit) = limits.get(&country) {
        let diff = limit - user.bac.as_float();
        if diff > 0.0 {
            let diff = diff.abs();
            let beers_left = diff / user.bac.beer_ac();
            println!(
                "You are {:.3} points under the legal driving limit of {}.",
                diff, country
            );
            println!(
                "It would take {:.2} beers to get to the legal limit.",
                beers_left
            );
        } else {
            let diff = diff.abs();
            println!(
                "You are {:.3} points over the legal driving limit of {}.",
                diff, country
            );
            {
                let (hours, minutes) = user.bac.hours_minutes_until(user.bac.as_float());
                println!(
                    "You need to not drink for {} hours and {} minutes to reach complete sobriety.",
                    hours.floor(),
                    minutes.round()
                );
            }
            {
                let (hours, minutes) = user.bac.hours_minutes_until(diff);
                println!(
                    "Or you could stop drinking for {} hours and {} minutes to legally drive.",
                    hours.floor(),
                    minutes.round()
                );
            }
        }
    }
}

fn person_report(user: &User) {
    println!("{}", user.bac.person.report());
}

fn cli_loop(user: User) -> Result<(), Error> {
    let mut buf = String::new();
    let effect_info = EffectInfo::from_path("data/effects.json")?;
    loop {
        println!("==============================");
        println!(
            "Your current blood alcohol concentration is {:.3}.",
            &user.bac.as_float()
        );
        println!("==============================");

        if let Some(effect) = effect_info.get_effect(user.bac.as_float()) {
            println!(
                "You may be experiencing the following behaviors:\n{}",
                effect.behaviors()
            );
        }

        drink_report(&user);
        println!("==============================");

        person_report(&user);
        println!("==============================");
        report_legal_limit(&user);
        println!("==============================");

        std::io::stdin().read_line(&mut buf)?;
        if !buf.is_empty() {
            break;
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let matches = get_matches();

    let dir_path = match &matches.value_of("config") {
        Some(config_path) => PathBuf::from(config_path),
        None => {
            let dir_path = current_dir()?;
            println!(
                "No config path provided. Using {}",
                dir_path.to_str().unwrap()
            );
            dir_path
        }
    };

    let user = User::open(dir_path)?;
    cli_loop(user)?;

    Ok(())
}
