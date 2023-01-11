// amcoder's solution is so much better! even after realizing that the
//  allergies are powers of 2, i did not think of bitwise operators!
// implemented their solution below
pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

use crate::Allergen::*;
#[rustfmt::skip]
const ALLERGENS : [Allergen; 8] = 
    [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score % 256 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.score & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .copied()
            .collect()
    }
}
