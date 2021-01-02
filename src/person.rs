use measurements::mass::Mass;

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

/// The person drinking alcohol
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
}