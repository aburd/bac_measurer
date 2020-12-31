use bac_journal::{BAC, beer_to_alcohol_mass};

fn main() {
    let drank_alc = beer_to_alcohol_mass(None, None);
    let bac = BAC::new(drank_alc, None);

    println!("Your BAC is {:.4}", &bac.as_float());
}
