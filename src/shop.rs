use std::fmt;

use crate::{card::Card, FightResult};

pub struct Shop {
    pub cards: Vec<Card>,
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    pub fn most_expensive(&self) -> u32 {
        self.cards.iter().map(|card| card.price).max().unwrap()
    }

    /// Get the total damage of all cards in the shop
    pub fn total_damage(&self) -> u32 {
        self.cards.iter().map(|card| card.damage).sum()
    }

    /// Get the total health of all cards in the shop
    pub fn total_health(&self) -> u32 {
        self.cards.iter().map(|card| card.health).sum()
    }

    /// Simulate a fight against another store. Returns a FightResult::Win if
    /// this store wins, FightResult::Loss if this store loses, and a
    /// FightResult::Tie if both stores win the same number of battles.
    pub fn fight_store(&self, other: &Shop) -> FightResult {
        
        let mut self_win = 0;
        let mut other_win = 0;

        for i in 0..self.cards.len() {
            match self.cards[i].fight(&other.cards[i]) {
                FightResult::Tie => {}
                FightResult::Win => {
                    self_win += 1;
                }
                FightResult::Loss => {
                    other_win += 1;
                }
                FightResult::Draw => {}
            }
        }

        match (self_win, other_win) {
            (self_win, other_win) if self_win > other_win => FightResult:: Win,
            (self_win, other_win) if other_win > self_win => FightResult:: Loss,
            (self_win, other_win) if self_win == other_win => FightResult::Tie,
            (_,_) => unreachable!(),
        }

    }
}

// Implement the Display trait for Shop so that it can be printed. Print the
// shop's stats, including the most expensive card, the total damage, and the
// total health.
impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|Shop: {}/{}/{}|",
            self.most_expensive(),
            self.total_damage(),
            self.total_health()
        )
    }
}
