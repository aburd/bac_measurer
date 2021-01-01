use measurements::mass::Mass;
use bac_journal::{BAC, Person, Gender};

fn is_close(val: f64, estimate: f64, sig_fig: u64) {
    let sf: f64 = sig_fig as f64;
    let val_trunc = (val * (sf * 100.0)).round() / (sf * 100.0);
    let estimate_trunc = (estimate * (sf * 100.0)).round() / (sf * 100.0);
    assert_eq!(val_trunc, estimate_trunc);
}

#[test]
fn it_approximates_males_correctly() {
    let person = Person::new(Gender::Male, 80.0);
    let bac = BAC::new(Mass::from_grams(30.0), Some(person));

    let answer = 0.0251;
    is_close(bac.as_float(), answer, 3);
}
