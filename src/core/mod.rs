pub mod player;
pub mod government;
pub mod policy;
pub mod policy_deck;
pub mod policy_tracker;

// THESE WON'T STAY HERE:

pub enum PresidentialPower {
    InvestigateLoyalty,
    CallSpecialElection,
    PolicyPeek,
    Execution,
}

pub enum GameStateChange {
    EnableVetoPower,
    LiberalPolicyWin,
    FascistPolicyWin,
}
