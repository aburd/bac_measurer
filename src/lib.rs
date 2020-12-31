extern crate measurements;
use measurements::mass::Mass;
use measurements::volume::Volume;

/// Blood alcohol content (BAC) is a measure of the amount of alcohol circulating in your bloodstream. 
/// It is expressed in terms of weight (milligrams) per unit of volume (milliliters) and is 
/// usually shown as a percentage. Blood alcohol content is used for legal and medical purposes 
/// to indicate a person's level of intoxication.  
/// Blood alcohol content is the amount of alcohol present in 100 milliliters (ml) 
/// or its equivalent of 1 deciliter (dL) of blood. For example: 
/// 
/// 80 mg is 0.08 grams
/// 0.08 grams of alcohol in 100 ml is 0.08%
/// This can also be expressed as 80 mg/dL or a BAC of 0.08


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
    fn body_water_ratio(&self) -> f64 {
        match self {
            Gender::Male => R_MALE,
            Gender::Female => R_FEMALE,
        }
    }

    fn metabolic_rate(&self) -> f64 {
        match self {
            Gender::Male => MET_RATE_M,
            Gender::Female => MET_RATE_F,
        }
    }
}

pub struct Person {
    gender: Gender,
    weight: Mass,
}

impl Person {
    pub fn new(gender: Gender, weight_kg: impl Into<f64>) -> Self {
        Person {
            gender,
            weight: Mass::from_kilograms(weight_kg.into()),
        }
    }
}

pub struct BAC {
    alcohol: Mass,
    pub person: Person,
}

impl BAC {
    pub fn new(alcohol: Mass, person: Option<Person>) -> Self {
        BAC {
            alcohol,
            person: person.unwrap_or(Person::new(Gender::Male, 60))
        }
    }

    /// Widmark Formula
    /// A / (r * Wt) - β * T
    /// A is the mass of alcohol consumed.
    /// r is the ratio of body water to total weight. It varies between individuals but averages about 0.68 for men and 0.55 for women, since women tend to a higher percentage of fat.
    /// Wt is body weight.
    /// β is the rate at which alcohol is metabolized. It is approximately 0.017% per hour.
    /// T is the amount time during which alcohol was present in the blood (usually time since consuption began).
    pub fn as_float(&self) -> f64 {
        let a = self.alcohol.as_grams();
        let r = self.person.gender.body_water_ratio();
        let wt = self.person.weight.as_grams();
        let b = self.person.gender.metabolic_rate();
        let t = 1.0;

        let bac_as_percent = a / (r * wt);
        let time_variance = b * t;
        (bac_as_percent * 100.0) - time_variance
    }
}

pub fn beer_to_alcohol_mass(percent: Option<f64>, ounces: Option<f64>) -> Mass {
    let beer_mass = Mass::from_ounces(ounces.unwrap_or(12.0));
    let alc_percent = percent.unwrap_or(5.0);
    beer_mass * (alc_percent / 100.0)
}