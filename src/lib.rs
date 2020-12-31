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
    weight_kg: u32,
    height_cm: u32,
    // phys_cond: PhysicalCond,
    // meal: Meal,
    // sleep_amt: Duration,
}

impl Person {
    pub fn new(gender: Option<Gender>, weight_kg: Option<u32>, height_cm: Option<u32>) -> Self {
        Person {
            gender: gender.unwrap_or(Gender::Unknown),
            weight_kg: weight_kg.unwrap_or(50),
            height_cm: height_cm.unwrap_or(50),
        }
    }

    pub fn blood_ml(&self) -> u32 {
        let avg_blood_of_person = 5 * 1000;
        avg_blood_of_person
    }
}

pub struct BAC {
    alcohol_g: f64,
    person: Person,
}

impl BAC {
    pub fn new(alcohol_g: f64, person: Option<Person>) -> Self {
        BAC {
            alcohol_g,
            person: person.unwrap_or(Person::new(None, None, None))
        }
    }

    pub fn get(&self) -> f64 {
        let blood_vol_ml = self.person.blood_ml();
        self.alcohol_g / (blood_vol_ml as f64 / 100.0)
    }
}

pub beers_to_alc_g(beer_num: u32) {

}