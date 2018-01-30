use super::policy::Policy;
use super::{PresidentialPower, GameStateChange};
use super::PresidentialPower::*;
use super::GameStateChange::*;


pub type PolicyEnactEffects = (Option<PresidentialPower>, Option<GameStateChange>);

pub struct PolicyTracker {
    player_count: u8,
    liberal_enacted: Vec<Policy>,
    fascist_enacted: Vec<Policy>,
}

impl PolicyTracker {
    pub fn new(player_count: u8) -> Self {
        Self {
            player_count: player_count,
            liberal_enacted: Vec::new(),
            fascist_enacted: Vec::new(),
        }
    }

    pub fn enact(&mut self, card: Policy) -> PolicyEnactEffects {
        match card {
            Policy::Liberal => {
                self.liberal_enacted.push(card);
                match self.liberal_enacted.len() {
                    5 => (None, Some(LiberalPolicyWin)),
                    _ => (None, None),
                }
            }
            Policy::Fascist => {
                self.fascist_enacted.push(card);
                match (self.fascist_enacted.len(), self.player_count) {
                    (1...2, 9...10) => (Some(InvestigateLoyalty), None),
                    (2, 7...8) => (Some(InvestigateLoyalty), None),
                    (3, 5...6) => (Some(PolicyPeek), None),
                    (3, 7...8) => (Some(CallSpecialElection), None),
                    (3, 9...10) => (Some(PolicyPeek), None),
                    (4, _) => (Some(Execution), None),
                    (5, _) => (Some(Execution), Some(EnableVetoPower)),
                    (6, _) => (None, Some(FascistPolicyWin)),
                    _ => (None, None),
                }
            }
        }
    }
}
