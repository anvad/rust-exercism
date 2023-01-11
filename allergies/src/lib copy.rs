pub struct Allergies {
    score: u32,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score % 256 }
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergies = self.allergies();
        allergies.contains(allergen)
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        let mut score = self.score;
        [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .iter()
        .map(|allergy| {
            if score % 2 > 0 {
                score /= 2;
                Some(*allergy)
            } else {
                score /= 2;
                None
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<Vec<_>>()
    }
}
