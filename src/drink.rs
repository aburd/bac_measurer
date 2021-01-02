use std::fs::{self, File};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use measurements::mass::Mass;

/// Holds data about an alcoholic drink
pub struct Drink {
    pub name: String,
    percent: f64,
    mass: Mass,
    datetime: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrinkJSON {
    name: String,
    percent: f64,
    grams: f64,
    datetime_rfc2822: String,
}

impl DrinkJSON {
    pub fn as_drink(self) -> Drink {
        let mass = Mass::from_grams(self.grams);
        let datetime = DateTime::parse_from_rfc2822(&self.datetime_rfc2822).unwrap().with_timezone(&chrono::Utc);
        
        Drink {
            name: self.name,
            percent: self.percent,
            mass,
            datetime,
        }
    }
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