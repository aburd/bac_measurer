extern crate measurements;
use crate::drink::{Drink, DrinkJSON};
use crate::person::{Gender, Person, PersonJSON};
use measurements::mass::Mass;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct BacJSON {
    drinks: Vec<DrinkJSON>,
    person: PersonJSON,
}

fn log_path(dir: &PathBuf) -> PathBuf {
    dir.join("config.json")
}

pub struct User {
    reader: BufReader<File>,
    pub bac: BAC,
}

impl User {
    pub fn open(dir_path: impl Into<PathBuf>) -> Result<Self, std::io::Error> {
        let dir_path = dir_path.into();
        std::fs::create_dir_all(&dir_path)?;
        let path = log_path(&dir_path);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                println!("{}", e);
                let mut f = File::create(&path)?;
                let bac_json = BacJSON {
                    drinks: vec![],
                    person: PersonJSON {
                        gender: String::from("male"),
                        grams: 70000.0,
                    },
                };
                let json = serde_json::to_string(&bac_json)?;
                f.write_all(json.as_bytes())?;
                File::open(&path)?
            }
        };

        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let bac_json: BacJSON = serde_json::from_str(&buf)?;

        let mut drinks: Vec<Drink> = vec![];
        for dj in bac_json.drinks {
            let drink = dj.as_drink().unwrap();
            drinks.push(drink);
        }
        let person = bac_json.person.as_person()?;

        let bac = BAC::new(drinks, person);

        Ok(User { reader, bac })
    }
}

/// Blood Alcohol Concentration
pub struct BAC {
    pub drinks: Vec<Drink>,
    pub person: Person,
}

impl BAC {
    /// Make a new BAC
    pub fn new(drinks: Vec<Drink>, person: Person) -> Self {
        BAC { drinks, person }
    }
    /// Add a drink to the user's session
    pub fn push_drink(&mut self, drink: Drink) {
        self.drinks.push(drink);
    }
    /// Specify a drink to add to the users session
    pub fn new_drink(&mut self, name: &str, percent: f64, mass: Mass) {
        let drink = Drink::new(name, percent, mass);
        self.push_drink(drink);
    }
    /// The first drink the user has had in the session
    pub fn report_first_drink(&self) {
        if let Some(drink) = self.drinks.first() {
            println!("You started drinking at: {}", drink.local_datetime());
        }
    }
    /// The number of drinks the user has had
    pub fn drink_len(&self) -> usize {
        self.drinks.len()
    }
    pub fn beer_ac(&self) -> f64 {
        let drink = Drink::from_beer("Beer");
        let person = self.person.clone();
        let bac = BAC::new(vec![drink], person);
        bac.as_float()
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

    /// The number of hours needed to reach a 0.0 BAC
    pub fn hours_till_0(&self) -> f64 {
        self.as_float() / self.person.gender.metabolic_rate()
    }
}
