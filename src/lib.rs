extern crate measurements;
use measurements::mass::Mass;

/// Holds data about an alcoholic drink
pub struct Drink {
    pub name: String,
    pub percent: f64,
    mass: Mass,
}

impl Drink {
    /// Create a drink where
    /// `name` is the name of the drink (for logging purposes)
    /// `percent` is the alcoholic percentage of the drink
    /// `mass` is the mass of the drink (e.g. 12oz)
    fn new(name: &str, percent: f64, mass: Mass) -> Self {
        Drink {
            name: name.to_string(),
            percent,
            mass,
        }
    }

    /// Create a drink as a common beer
    fn from_beer(name: &str) -> Self {
        Drink {
            name: name.to_string(),
            percent: 5.0,
            mass: Mass::from_ounces(12.0),
        }
    }

    /// A function that reports the alcoholic mass
    /// of the drink
    pub fn alcohol_mass(&self) -> Mass {
        self.mass * (self.percent / 100.0)
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
        // TODO: select or ask for times within a range
        let t = 2.0;

        let bac_as_percent = a / (r * wt);
        let time_variance = b * t;
        (bac_as_percent * 100.0) - time_variance
    }
}
