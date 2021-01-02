use measurements::mass::Mass;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Used to show the gender of the alcohol user
#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
        }
    }
}

// NOTE: https://en.wikipedia.org/wiki/Blood_alcohol_content#Estimation_by_intake
const R_MALE: f64 = 0.68;
const R_FEMALE: f64 = 0.55;
const MET_RATE_M: f64 = 0.015;
const MET_RATE_F: f64 = 0.017;

impl Gender {
    /// The body water ratio for each gender
    pub fn body_water_ratio(&self) -> f64 {
        match self {
            Gender::Male => R_MALE,
            Gender::Female => R_FEMALE,
        }
    }

    /// The metabolic rate for each gender
    pub fn metabolic_rate(&self) -> f64 {
        match self {
            Gender::Male => MET_RATE_M,
            Gender::Female => MET_RATE_F,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonJSON {
    pub gender: String,
    pub grams: f64,
}

impl PersonJSON {
    pub fn as_person(self) -> Result<Person, std::io::Error> {
        let weight = Mass::from_grams(self.grams);
        let gender: Gender = if self.gender == String::from("male") {
            Gender::Male
        } else if self.gender == String::from("female") {
            Gender::Female
        } else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        };

        Ok(Person { weight, gender })
    }
}

/// The person drinking alcohol
#[derive(Clone)]
pub struct Person {
    pub gender: Gender,
    pub weight: Mass,
}

impl Person {
    /// Create new person
    pub fn new(gender: Gender, weight_kg: impl Into<f64>) -> Self {
        Person {
            gender,
            weight: Mass::from_kilograms(weight_kg.into()),
        }
    }

    pub fn report(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "You are a {} and weigh {:.2} kgs ({:.2} lbs).",
            self.gender,
            self.weight.as_kilograms(),
            self.weight.as_pounds(),
        ));
        s
    }
}
