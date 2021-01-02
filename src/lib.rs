extern crate measurements;
use measurements::mass::Mass;
use chrono::prelude::*;

/// Holds data about an alcoholic drink
pub struct Drink {
    pub name: String,
    percent: f64,
    mass: Mass,
    datetime: DateTime<Utc>,
}

impl Drink {
    /// Create a drink where
    /// `name` is the name of the drink (for logging purposes)
    /// `percent` is the alcoholic percentage of the drink
    /// `mass` is the mass of the drink (e.g. 12oz)
    pub fn new(name: &str, percent: f64, mass: Mass) -> Self {
        Drink {
            name: name.to_string(),
            percent,
            mass,
            datetime: Utc::now(),
        }
    }
    
    pub fn new_with_date(name: &str, percent: f64, mass: Mass, datetime: DateTime<Utc>) -> Self {
        Drink {
            name: name.to_string(),
            percent,
            mass,
            datetime,
        }
    }

    /// Create a drink as a common beer
    pub fn from_beer(name: &str) -> Self {
        Drink {
            name: name.to_string(),
            percent: 5.0,
            mass: Mass::from_ounces(12.0),
            datetime: Utc::now(),
        }
    }

    /// A function that reports the alcoholic mass
    /// of the drink
    pub fn alcohol_mass(&self) -> Mass {
        self.mass * (self.percent / 100.0)
    }

    pub fn hours_ago(&self) -> f64 {
        let now = Utc::now();
        let duration = now - self.datetime;
        let seconds = duration.num_seconds();
        seconds as f64 / (60.0 * 60.0)
    }
}

/// Used to show the gender of the alcohol user
pub enum Gender {
    Male,
    Female,
}

// NOTE: https://en.wikipedia.org/wiki/Blood_alcohol_content#Estimation_by_intake
const R_MALE: f64 = 0.68;
const R_FEMALE: f64 = 0.55;
const MET_RATE_M: f64 = 0.015;
const MET_RATE_F: f64 = 0.017;

impl Gender {
    /// The body water ratio for each gender
    fn body_water_ratio(&self) -> f64 {
        match self {
            Gender::Male => R_MALE,
            Gender::Female => R_FEMALE,
        }
    }

    /// The metabolic rate for each gender
    fn metabolic_rate(&self) -> f64 {
        match self {
            Gender::Male => MET_RATE_M,
            Gender::Female => MET_RATE_F,
        }
    }
}

/// The person drinking alcohol
pub struct Person {
    gender: Gender,
    weight: Mass,
}

impl Person {
    /// Create new person
    pub fn new(gender: Gender, weight_kg: impl Into<f64>) -> Self {
        Person {
            gender,
            weight: Mass::from_kilograms(weight_kg.into()),
        }
    }
}

/// Blood Alcohol Concentration
pub struct BAC {
    drinks: Vec<Drink>,
    pub person: Person,
}

impl BAC {
    pub fn new(person: Option<Person>) -> Self {
        BAC {
            drinks: Vec::new(),
            person: person.unwrap_or(Person::new(Gender::Male, 60))
        }
    }

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
            let a = self.drinks.iter()
                .map(|d| d.alcohol_mass().as_grams())
                .fold(0.0, |total, mass| total + mass);
            let r = self.person.gender.body_water_ratio();
            let wt = self.person.weight.as_grams();
            let b = self.person.gender.metabolic_rate();
            
            let t = first_drink.hours_ago();
    
            let bac_as_percent = a / (r * wt);
            let time_variance = b * t;
            return (bac_as_percent * 100.0) - time_variance
        }
        0.0
    }
}
