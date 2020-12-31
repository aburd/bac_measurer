use bac_journal::{BAC, beer_to_alcohol_mass};

fn main() {
    let drank_alc = beer_to_alcohol_mass(None, None);
    let bac = BAC::new(drank_alc * 2.0, None);

    println!("You drank 1 beer, which is {} grams of alcohol", drank_alc.as_grams());
    println!("Your BAC is {:.4}", &bac.as_float());
}
