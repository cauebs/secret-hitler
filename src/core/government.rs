use std::collections::HashMap;

use super::player::{Player, PlayerID};


pub type Government = (Player, Player);

pub struct GovernmentTracker {
    history: Vec<Government>,
}

impl GovernmentTracker {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
}

pub enum Vote {
    Ja,
    Nein,
}

#[derive(Debug)]
pub enum Error {
    PlayerAlreadyVoted(PlayerID),
}


pub struct ElectionTracker {
    player_count: u8,
    current_poll: HashMap<PlayerID, Vote>,
    consecutive_fails: u8,
}

impl ElectionTracker {
    pub fn new(player_count: u8) -> Self {
        Self {
            player_count: player_count,
            current_poll: HashMap::with_capacity(player_count as usize),
            consecutive_fails: 0,
        }
    }

    pub fn vote(&mut self, player: &Player, vote: Vote) -> Result<(), Error> {
        match self.current_poll.insert(player.id, vote) {
            Some(_) => Err(Error::PlayerAlreadyVoted(player.id)),
            None => Ok(())
        }
    }
}
