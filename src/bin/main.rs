use bac_journal::drink::legal_limits;
use bac_journal::User;
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

    if let Some(drink) = user.bac.first_drink() {
        let datetime = drink.datetime.with_timezone(&chrono::Local);
        println!("Your first drink was at {}.", datetime);
        println!(
            "The drink was {} oz at a percentage of {}.",
            drink.mass.as_ounces(),
            drink.percent
        );
        println!(
            "The drink had an alcoholic mass of {} grams.",
            drink.alcohol_mass().as_grams()
        );
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
                "You are {:.3} points under the legal driving limit of {}",
                diff, country
            );
            println!(
                "It would take {:.2} beers to get to the legal limit",
                beers_left
            );
        } else {
            let diff = diff.abs();
            println!(
                "You are {:.3} points over the legal driving limit of {}",
                diff, country
            );
            let hours = user.bac.hours_till_0();
            let minutes = (hours - hours.floor()) * 60.0;
            println!(
                "You need to not drink for {} hours and {} minutes to reach complete sobriety.",
                hours.floor(),
                minutes.round()
            );
        }
    }
}

fn person_report(user: &User) {
    println!(
        "You are a {} and weigh {} kgs.",
        user.bac.person.gender,
        user.bac.person.weight.as_kilograms()
    );
}

fn cli_loop(user: User) {
    let mut buf = String::new();
    loop {
        println!("Your current BAC is {:.3}", &user.bac.as_float());
        println!("\n\n");

        drink_report(&user);
        person_report(&user);
        report_legal_limit(&user);

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
            println!(
                "No config path provided. Using {}",
                dir_path.to_str().unwrap()
            );
            dir_path
        }
    };

    let user = User::open(dir_path)?;
    cli_loop(user);

    Ok(())
}
