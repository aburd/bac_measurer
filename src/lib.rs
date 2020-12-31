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
    Unknown,
}

pub struct Person {
    gender: Gender,
    weight: u32,
    height: u32,
    // phys_cond: PhysicalCond,
    // meal: Meal,
    // sleep_amt: Duration,
}

impl Person {
    pub fn new(gender: Option<Gender>, weight: Option<u32>, height: Option<u32>) -> Self {
        Person {
            gender: gender.unwrap_or(Gender::Unknown),
            weight: weight.unwrap_or(50),
            height: height.unwrap_or(50),
        }
    }

    pub fn blood_as_ml(&self) -> Volume {
        let avg_blood_of_person = 5.0 * 1000.0;
        Volume::from_milliliters(avg_blood_of_person)
    }
}

pub struct BAC {
    alcohol_g: f64,
    pub person: Person,
}

impl BAC {
    pub fn new(alcohol_g: f64, person: Option<Person>) -> Self {
        BAC {
            alcohol_g,
            person: person.unwrap_or(Person::new(None, None, None))
        }
    }

    pub fn as_float(&self) -> f64 {
        let blood_vol = self.person.blood_as_ml();
        self.alcohol_g / (blood_vol.as_milliliters() / 100.0)
    }
}

pub fn beer_to_alc_g(percent: Option<f64>, ounces: Option<f64>) -> f64 {
    let beer_mass = Mass::from_ounces(ounces.unwrap_or(12.0));
    let alc_percent = percent.unwrap_or(5.0);
    beer_mass.as_grams() * (alc_percent / 100.0)
}