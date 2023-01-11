// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

// koushik-ms's solution is really good

impl Player {
    pub fn new(level: u32) -> Self {
        Self {
            health: 100,
            mana: if level < 10 { None } else { Some(100) },
            level,
        }
    }
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Self::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                // this ensures health does not turn negative
                self.health -= min(self.health, mana_cost);
                0 // no need for return statement, as this will be the
                  //  last expression evaluated
            }
            Some(mana) => {
                if mana < mana_cost {
                    0
                } else {
                    self.mana = Some(mana - mana_cost);
                    2 * mana_cost // no need for return statement
                }
            }
        }
    }
}
