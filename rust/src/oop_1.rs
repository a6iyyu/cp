#![warn(dead_code)]

pub enum SexTypes {
  Feminine,
  Masculine,
  Hermaphrodite,
}

struct Physique;
impl Physique {
	pub fn new(fat: usize, muscle: usize) -> &'static str {
    let body_types: [[&'static str; 7]; 7] = [
      ["Emaciated", "Thin", "Lean", "Toned", "Defined", "Sculpted", "Ripped"],
      ["Gaunt", "Slender", "Slim", "Fit", "Taut", "Cut", "Shredded"],
      ["Underweight", "Svelte", "Lithe", "Athletic", "Muscular", "Built", "Solid"],
      ["Average", "Wiry", "Balanced", "Stocky", "Husky", "Buff", "Hefty"],
      ["Soft", "Slightly Firm", "Firm", "Bulky", "Strong", "Burly", "Massive"],
      ["Pudgy", "Chubby", "Stout", "Robust", "Thick", "Powerhouse", "Brawny"],
      ["Overweight", "Portly", "Heavyset", "Large", "Hulking", "Gargantuan", "Herculean"],
    ];

    if fat < 4 && muscle < 4 {
      body_types[fat][muscle]
    } else {
      "Invalid body type!"
    }
  }
}

pub struct Character {
  id: String,
  forename: String,
  surname: String,
  sex_type: SexTypes,
  age: i32,
  fat: usize,
  muscle: usize,
  physique: String,
}

impl Character {
  pub fn new(id: &str, forename: &str, surname: &str, sex_type: SexTypes, age: i32, fat: usize, muscle: usize) -> Self {
    let physique = Physique::new(fat, muscle).to_string();
    Character {
      id: id.to_string(),
      forename: forename.to_string(),
      surname: surname.to_string(),
      sex_type,
      age,
      fat,
      muscle,
      physique,
    }
  }

  pub fn describe_character(&self) -> String {
    let gender_description = match self.sex_type {
      SexTypes::Feminine => {
        if self.age <= 21 {
          "Girl"
        } else {
          "Woman"
        }
      }
      SexTypes::Masculine => {
        if self.age <= 21 {
          "Boy"
        } else {
          "Man"
        }
      }
      SexTypes::Hermaphrodite => "Hermaphrodite",
    };
    format!("A {} {}", self.physique, gender_description)
  }
}