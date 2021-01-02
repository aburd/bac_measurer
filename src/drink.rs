use chrono::prelude::*;
use measurements::mass::Mass;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Holds data about an alcoholic drink
pub struct Drink {
    pub name: String,
    pub percent: f64,
    pub mass: Mass,
    pub datetime: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrinkJSON {
    name: String,
    percent: f64,
    grams: f64,
    datetime_rfc2822: String,
}

impl DrinkJSON {
    pub fn as_drink(self) -> Result<Drink, std::io::Error> {
        let mass = Mass::from_grams(self.grams);
        let datetime_fixed = DateTime::parse_from_rfc2822(&self.datetime_rfc2822).unwrap();
        let datetime = datetime_fixed.with_timezone(&chrono::Utc);

        Ok(Drink {
            name: self.name,
            percent: self.percent,
            mass,
            datetime,
        })
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

    pub fn local_datetime(&self) -> DateTime<Local> {
        self.datetime.with_timezone(&Local)
    }

    pub fn report(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!(
            "You drank a {} at {}\n",
            self.name,
            self.local_datetime()
        ));
        s.push_str(&format!(
            "That drink was {} grams ({} oz) at a percentage of {}.\n",
            self.mass.as_grams().round(),
            self.mass.as_ounces().round(),
            self.percent
        ));
        s.push_str(&format!(
            "The drink had a pure alcohol mass of {} grams.",
            self.alcohol_mass().as_grams().round()
        ));
        s
    }
}

type Limits = HashMap<String, f64>;

pub fn legal_limits() -> Limits {
    let mut limits: Limits = HashMap::new();
    limits.insert("China".to_string(), 0.02);
    limits.insert("United States".to_string(), 0.02);
    limits.insert("Japan".to_string(), 0.03);
    limits.insert("India".to_string(), 0.03);

    limits
}
