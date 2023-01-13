#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// technically, all we need to store is the sequence of pins rolled
//  and number of frames, score in each frame, whether game is done can
//  be calculated from that
// but, i am adding redundancy (memoization?) to make it easier to navigate
//  so that i don't have to recalculate frame/score after each roll
pub struct BowlingGame {
    // game is done when frames.len() == 10 and every frame is complete
    frames: Vec<Frame>,
}

// score of a frame is sum of rolls
// if strike, then 2nd and 3rd rolls are really fill slots
// if spare, then 3rd roll is really fill slot
// if neither strike nor spare, then fill slot will be set to 0
// frame is considered done when rolls.len() == 3
struct Frame {
    rolls: Vec<u16>, // always a vec of size 3 when complete
    state: FrameState,
}

#[derive(PartialEq)]
enum FrameState {
    InProgress,    // set to InProgress when previous Frame is NOT InProgress
    StrikeOrSpare, // set to this after a strike or spare
    Complete,      // set to Complete after fill slots are filled
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: vec![] }
    }

    fn is_game_done(&self) -> bool {
        self.frames.len() == 10 && self.frames.iter().all(|f| f.state == FrameState::Complete)
    }

    // roll can be 1st or 2nd roll of a frame
    //  or it could be fill-ball1 or fill-ball2
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        use crate::FrameState::*;
        if self.is_game_done() {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // get current frame (creating it if necessary)
        let cur_frame_index = self.get_current_frame();
        if cur_frame_index.is_none() {
            // check to make sure we had enough pins
            return Ok(());
        }

        // update previous frames that are in StrikeOrSpare state
        self.frames
            .iter_mut()
            .filter(|f| f.state == StrikeOrSpare)
            .for_each(|f| f.rolls.push(pins));
        self.frames
            .iter_mut()
            .filter(|f| f.state == StrikeOrSpare && f.rolls.len() == 3)
            .for_each(|f| f.state = Complete);

        // update pins for cur_frame
        // at this point, cur_frame will be in InProgress state
        let mut cur_frame = &mut self.frames[cur_frame_index.unwrap()];
        let sum = cur_frame.rolls.get(0).unwrap_or(&0) + pins;
        if sum > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        cur_frame.rolls.push(pins);

        // finally update state of current frame
        if sum == 10 {
            cur_frame.state = StrikeOrSpare;
        } else if cur_frame.rolls.len() == 2 {
            cur_frame.state = Complete;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_done() {
            Some(self.frames.iter().flat_map(|f| f.rolls.iter()).sum())
        } else {
            None
        }
    }

    /// returns index of current frame unless all frames are played out
    fn get_current_frame(&mut self) -> Option<usize> {
        use crate::FrameState::*;
        let num_frames = self.frames.len();
        match self.frames.last() {
            Some(frame) if frame.state == InProgress => Some(num_frames - 1),
            _ if num_frames == 10 => None, // all 10 frames have been played
            _ => {
                let frame = Frame {
                    rolls: vec![],
                    state: InProgress,
                };
                self.frames.push(frame);
                Some(num_frames)
            }
        }
    }
}
