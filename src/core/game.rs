use rand::{thread_rng, Rng};

use super::policy_deck::PolicyDeck;
use super::policy_tracker::PolicyTracker;
use super::government::{GovernmentTracker, ElectionTracker};
use super::player::{Player, SecretRole};

#[derive(Debug)]
pub enum Error {
    TooFewPlayers,
    TooManyPlayers,
}

pub struct Game {
    pub players: Vec<Player>,
    policy_deck: PolicyDeck,
    policy_tracker: PolicyTracker,
    government_tracker: GovernmentTracker,
    election_tracker: ElectionTracker,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Result<Self, Error> {
        let player_count = players.len() as u8;
        match player_count {
            0...4 => Err(Error::TooFewPlayers),
            5...10 => Ok(Self {
                players: players,
                policy_deck: PolicyDeck::new(),
                policy_tracker: PolicyTracker::new(player_count),
                government_tracker: GovernmentTracker::new(),
                election_tracker: ElectionTracker::new(player_count),
            }),
            _ => Err(Error::TooManyPlayers),
        }
    }

    pub fn start(&mut self) {
        self.assign_roles();
    }

    fn assign_roles(&mut self) {
        let player_count = self.players.len();
        let (liberal_count, fascist_count) = match player_count {
            5 => (3, 2),
            6 => (4, 2),
            7 => (4, 3),
            8 => (5, 3),
            9 => (5, 4),
            10 => (6, 4),
            _ => panic!("Can't start game with invalid number of players!"),
        };

        let mut roles = vec![SecretRole::Hitler];
        for _ in 0..liberal_count {
            roles.push(SecretRole::Liberal);
        }
        for _ in 0..(fascist_count - 1) {
            roles.push(SecretRole::Fascist);
        }

        let mut rng = thread_rng();
        rng.shuffle(&mut roles);

        for (mut player, role) in self.players.iter_mut().zip(roles) {
            player.secret_role = Some(role);
        }
    }
}
