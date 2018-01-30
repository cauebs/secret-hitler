pub enum Party {
    Liberal,
    Fascist,
}

pub enum SecretRole {
    Liberal,
    Fascist,
    Hitler,
}

pub struct PlayerTracker {
    party_membership: Party,
    secret_role: SecretRole,
    not_hitler_confirmed: bool,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct PlayerID(usize);

pub struct Player {
    pub id: PlayerID,
    name: String,
    info: Option<PlayerTracker>,
}

impl Player {

}
