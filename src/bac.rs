extern crate measurements;
use measurements::mass::Mass;

use crate::drink::{Drink};
use crate::person::{Gender, Person};

/// Blood Alcohol Concentration
pub struct BAC {
    drinks: Vec<Drink>,
    pub person: Person,
}

impl BAC {
    pub fn new(person: Option<Person>) -> Self {
        BAC {
            drinks: Vec::new(),
            person: person.unwrap_or(Person::new(Gender::Male, 60)),
        }
    }

    // pub fn from_config(path: &str) -> Result<Self, std::io::Error> {
    //     // let file = File::open(path)?;

    //     let drink_json: Vec<DrinkJSON> = serde_json::from_str(data).unwrap();
    //     let drinks: Vec<_> = drink_json.iter().map(|dj| dj.as_drink()).collect();
    //     Ok(BAC {
    //         drinks
    //     })
    // }

    pub fn push_drink(&mut self, drink: Drink) {
        self.drinks.push(drink);
    }

    pub fn new_drink(&mut self, name: &str, percent: f64, mass: Mass) {
        let drink = Drink::new(name, percent, mass);
        self.push_drink(drink);
    }

    /// Widmark Formula
    /// A / (r * Wt) - β * T
    /// A is the mass of alcohol consumed.
    /// r is the ratio of body water to total weight. It varies between individuals but averages about 0.68 for men and 0.55 for women, since women tend to a higher percentage of fat.
    /// Wt is body weight.
    /// β is the rate at which alcohol is metabolized. It is approximately 0.017% per hour.
    /// T is the amount time during which alcohol was present in the blood (usually time since consuption began).
    pub fn as_float(&self) -> f64 {
        if let Some(first_drink) = self.drinks.first() {
            let a = self
                .drinks
                .iter()
                .map(|d| d.alcohol_mass().as_grams())
                .fold(0.0, |total, mass| total + mass);
            let r = self.person.gender.body_water_ratio();
            let wt = self.person.weight.as_grams();
            let b = self.person.gender.metabolic_rate();

            let t = first_drink.hours_ago();

            let bac_as_percent = a / (r * wt);
            let time_variance = b * t;
            return (bac_as_percent * 100.0) - time_variance;
        }
        0.0
    }
}
