use bac_journal::{BAC, beer_to_alc_g};

fn main() {
    let drank_alc = beer_to_alc_g(None, None);
    let bac = BAC::new(drank_alc, None);

    println!("You drank 1 beer.");
    println!("You have a blood volume of {}", &bac.person.blood_as_ml());
    println!("Your BAC is {:.2}", &bac.as_float());
}
