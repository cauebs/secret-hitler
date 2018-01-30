#[derive(Clone)]
pub enum Policy {
    Liberal,
    Fascist,
}

impl Policy {
    fn is_liberal(&self) -> bool {
        match self {
            &Policy::Liberal => true,
            &Policy::Fascist => false,
        }
    }

    fn is_fascist(&self) -> bool {
        !self.is_liberal()
    }
}
