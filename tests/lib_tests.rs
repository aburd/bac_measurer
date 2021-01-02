use chrono::prelude::*;
use chrono::Duration;
use measurements::mass::Mass;
use bac_journal::{BAC, Person, Gender, Drink};

fn is_close(val: f64, estimate: f64, sig_fig: u64) {
    let sf: f64 = sig_fig as f64;
    let val_trunc = (val * (sf * 100.0)).round() / (sf * 100.0);
    let estimate_trunc = (estimate * (sf * 100.0)).round() / (sf * 100.0);
    assert_eq!(val_trunc, estimate_trunc);
}

#[test]
fn it_approximates_males_correctly() {
    let drink = Drink::from_beer("Bud");
    let person = Person::new(Gender::Male, 80.0);
    let mut bac = BAC::new(Some(person));
    bac.push_drink(drink);

    let answer = 0.0313;
    is_close(bac.as_float(), answer, 3);
}

#[test]
fn it_approximates_females_correctly() {
    let now = Utc::now();
    let two_hours = Duration::hours(2);
    let two_hour_ago = now - two_hours;
    let drink = Drink::new_with_date(
        "Everclear",
        100.0,
        Mass::from_grams(2.5 * 10.0),
        two_hour_ago,
    );
    // 70 kg woman drinking 2.5 drinks of 10 grams each, in two hours:
    let person = Person::new(Gender::Female, 70.0);
    let mut bac = BAC::new(Some(person));
    bac.push_drink(drink);

    let answer = 0.03;
    is_close(bac.as_float(), answer, 3);
}
