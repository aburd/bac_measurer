use measurements::mass::Mass;
use bac_journal::{BAC, Person, Gender};

fn main() {
    let person = Person::new(Gender::Male, 80.0);
    let bac = BAC::new(Mass::from_grams(30.0), Some(person));

    println!("Your BAC is {:.4}", &bac.as_float());
}
