use bac_journal::drink::Drink;
use bac_journal::{
    person::{Gender, Person},
    BAC,
};
use measurements::mass::Mass;

fn is_close(val: f64, estimate: f64, sig_fig: u64) {
    let sf: f64 = sig_fig as f64;
    let val_trunc = (val * (sf * 100.0)).round() / (sf * 100.0);
    let estimate_trunc = (estimate * (sf * 100.0)).round() / (sf * 100.0);
    assert_eq!(val_trunc, estimate_trunc);
}

fn drink_test(gender: Gender, alc_ounces: f64, weight: f64, answer: f64) {
    let drink = Drink::new("Liquor", 100.0, Mass::from_ounces(alc_ounces));
    let person = Person::new(gender, weight);
    let drinks = vec![drink];
    let bac = BAC::new(drinks, person);

    is_close(bac.as_float(), answer, 3);
}

#[test]
fn male_45_kg() {
    drink_test(Gender::Male, 0.5, 45.0, 0.046);
}
#[test]
fn male_55_kg() {
    drink_test(Gender::Male, 0.5, 55.0, 0.036);
}
#[test]
fn male_64_kg() {
    drink_test(Gender::Male, 0.5, 64.0, 0.033);
}

#[test]
fn female_45_kg() {
    drink_test(Gender::Female, 0.5, 45.0, 0.056);
}
#[test]
fn female_55_kg() {
    drink_test(Gender::Female, 0.5, 55.0, 0.046);
}
#[test]
fn female_64_kg() {
    drink_test(Gender::Female, 0.5, 64.0, 0.04);
}
