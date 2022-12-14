pub struct Allergies {
    allergen_hold: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        /* unimplemented!(
            "Given the '{}' score, construct a new Allergies struct.",
            score
        ); */

        let all_allergen = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        let u8score: u8 = (score % 256) as u8;

        Self {
            allergen_hold: all_allergen
                .into_iter()
                .filter(|a| (u8score & *a as u8) > 0)
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        /* unimplemented!(
            "Determine if the patient is allergic to the '{:?}' allergen.",
            allergen
        ); */
        self.allergen_hold.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        /* unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made."); */
        self.allergen_hold.clone()
    }
}
