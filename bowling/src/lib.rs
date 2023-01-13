#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// all we need to store is the sequence of pins rolled
//  and number of frames, score in each frame, whether game is done can
//  be calculated from that

#[derive(Debug)]
pub struct BowlingGame {
    rolls: Vec<Roll>, // list of all rolls played so far
    rolls_left: i16,  // max. number of possible rolls left at any point
    pins_left: u16,   // pins remaining after last roll, range is 1..=10
}

// the number stores the number of pins that fell in this roll
#[derive(PartialEq, Debug)]
enum Roll {
    First(u16), // first roll, so frame is not done yet
    Open(u16),  // after 2nd roll, if pins are still standing, then we have an open frame
    Strike,     // frame is done after just one roll!
    Spare(u16), // frame is complete, but needed two rolls
    Bonus(u16), // Bonus ball after all frames are done
}

use Roll::*;

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: vec![],
            rolls_left: 20,
            pins_left: 10,
        }
    }

    fn is_game_done(&self) -> bool {
        self.rolls_left < 1
    }

    // `rolls_diff` is how many roll count to subtract from `self.rolls_left`
    // `pins` is how many pins to remove from the lane (i.e. `self.pins_left`)
    fn append_roll(&mut self, roll: Roll, rolls_diff: i16, pins: u16) -> Result<(), Error> {
        self.pins_left = if self.pins_left == pins {
            10
        } else {
            self.pins_left - pins
        };
        self.rolls_left += rolls_diff;
        self.rolls.push(roll);
        Ok(())
    }

    // returns the number of completed frames
    fn frames(&self) -> u8 {
        self.rolls
            .iter()
            .map(|r| match *r {
                Roll::First(_) => 0,
                Roll::Bonus(_) => 0,
                _ => 1,
            })
            .sum()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_done() {
            return Err(Error::GameComplete);
        }
        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }
        let frames_played = self.frames();
        match self.rolls.last() {
            Some(First(_)) if pins == self.pins_left && frames_played == 9 => {
                self.append_roll(Spare(pins), 0, pins)
            }
            Some(First(_)) if pins == self.pins_left => self.append_roll(Spare(pins), -1, pins),
            Some(First(_)) => self.append_roll(Open(pins), -1, self.pins_left),

            // None or Spare or Open or Strike => Strike or First() or Bonus()
            _ if frames_played == 10 => self.append_roll(Bonus(pins), -1, pins),
            _ if pins == 10 && frames_played == 9 => self.append_roll(Strike, 0, pins),
            _ if pins == 10 => self.append_roll(Strike, -2, pins),
            _ => self.append_roll(First(pins), -1, pins),
        }
    }

    fn roll_points(&self, i: usize) -> u16 {
        let roll = &self.rolls.get(i);
        let bonus1 = &self.rolls.get(i + 1);
        let bonus2 = &self.rolls.get(i + 2);
        match (*roll, *bonus1, *bonus2) {
            (Some(Strike), Some(f1), Some(f2)) => 10 + roll_pins(f1) + roll_pins(f2),
            (Some(Spare(p)), Some(f1), _) => *p + roll_pins(f1),
            (Some(Open(p)), ..) => *p,
            (Some(First(p)), ..) => *p,
            _ => 0,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_done() {
            let sum = (0..self.rolls.len()).map(|i| self.roll_points(i)).sum();
            Some(sum)
        } else {
            None
        }
    }
}

// given a &Roll, returns the number of pins that fell
fn roll_pins(roll: &Roll) -> u16 {
    match *roll {
        First(p) => p,
        Open(p) => p,
        Spare(p) => p,
        Strike => 10,
        Bonus(p) => p,
    }
}
