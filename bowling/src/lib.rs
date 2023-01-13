#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// potatosalad's solution is so neat! Copying his solution below.
// His basic idea (store the rolls) is similar to mine.
// Where he saved a lot of complexity is by simply storing numbers.
// Since i stored enums, i had to spend a lot of code, trying to extract the numbers.
// Another clever idea he used is to use the `score` function to determine game end!
// Finally, i learnt something new about rust- using Default to create a new instance of a struct.
#[derive(Debug, Default)]
pub struct BowlingGame {
    rolls: Vec<u16>, // list of pins that fell on each roll played so far
    second: bool, // whether next roll will be the 2nd roll (pins are not reset before the 2nd roll)
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score().is_some() {
            return Err(Error::GameComplete);
        }
        if pins > 10 || (self.second && self.rolls.last().unwrap() + pins > 10) {
            return Err(Error::NotEnoughPinsLeft);
        }
        // if this roll is a strike, then next roll is a first (i.e. not second)
        // else, next roll toggles between first and second
        if pins == 10 {
            self.second = false;
        } else {
            self.second = !self.second;
        }
        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // make sure 10 frames are rolled, and any bonus rolls after that.
        let mut sum = 0;
        let mut i = 0;
        for _ in 0..10 {
            let first = *self.rolls.get(i)?;
            if first == 10 {
                let (bonus1, bonus2) = (*self.rolls.get(i + 1)?, *self.rolls.get(i + 2)?);
                sum += first + bonus1 + bonus2;
                i += 1;
                continue;
            }
            let second = *self.rolls.get(i + 1)?;
            sum += first + second;
            if first + second == 10 {
                let bonus1 = *self.rolls.get(i + 2)?;
                sum += bonus1;
            }
            i += 2;
        }
        Some(sum)
    }
}
