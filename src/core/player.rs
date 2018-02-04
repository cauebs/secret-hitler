#[derive(Debug)]
pub enum SecretRole {
    Liberal,
    Fascist,
    Hitler,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct PlayerID(pub usize);

pub struct Player {
    pub id: PlayerID,
    pub name: String,
    pub secret_role: Option<SecretRole>,
    pub not_hitler_confirmed: bool,
}

impl Player {
    pub fn new(id: PlayerID, name: String) -> Self {
        Self {
            id: id,
            name: name,
            secret_role: None,
            not_hitler_confirmed: false,
        }
    }
}
