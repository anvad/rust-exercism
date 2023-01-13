#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// potatosalad's solution is so neat! Copying his solution below.
// His basic idea (store the rolls) is similar to mine.
// Where he saved a lot of complexity is by simply storing numbers.
// Since i originally stored enums, i had to spend a lot of code, trying to extract the numbers.
// Another clever idea he used is to use the `score` function to determine game end!
// I learnt something new about rust- using Default to create a new instance of a struct.
// I used `?` to short-circuit `score()` when getting optional values.
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
        self.rolls.push(pins);
        // if this roll is a strike, or this roll is the second, then next roll is the first
        self.second = !(pins == 10 || self.second);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // make sure 10 frames are rolled, and any bonus rolls after that.
        // for scoring, every "frame" needs at least two rolls. For strike, we need three.
        let mut sum = 0;
        let mut roll_index = 0;
        for _ in 0..10 {
            let first = *self.rolls.get(roll_index)?;
            let second_or_bonus1 = *self.rolls.get(roll_index + 1)?;
            sum += first + second_or_bonus1;
            if first == 10 {
                let bonus2 = *self.rolls.get(roll_index + 2)?;
                sum += bonus2;
                roll_index += 1; // only for strike, we advance frame by one roll
                continue;
            }
            if first + second_or_bonus1 == 10 {
                let bonus1 = *self.rolls.get(roll_index + 2)?;
                sum += bonus1;
            }
            roll_index += 2; // we advance frame by two rolls
        }
        Some(sum)
    }
}
