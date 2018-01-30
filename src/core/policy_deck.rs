use rand::{thread_rng, Rng};

use std::mem;

use super::policy::Policy;


pub struct PolicyDeck {
    draw_pile: Vec<Policy>,
    discard_pile: Vec<Policy>,
}

impl PolicyDeck {
    pub fn new() -> Self {
        let mut cards = vec![Policy::Liberal; 11];
        cards.extend(vec![Policy::Fascist; 6]);

        let mut rng = thread_rng();
        rng.shuffle(&mut cards);

        Self {
            draw_pile: cards,
            discard_pile: Vec::new(),
        }
    }

    fn reshuffle(&mut self) {
        let discarded = mem::replace(&mut self.discard_pile, Vec::new());
        self.draw_pile.extend(discarded);
        let mut rng = thread_rng();
        rng.shuffle(&mut self.draw_pile);
    }

    pub fn draw(&mut self) -> Option<Policy> {
        let card = self.draw_pile.pop()?;
        if self.draw_pile.len() < 3 {
            self.reshuffle();
        }
        Some(card)
    }

    pub fn discard(&mut self, card: Policy) {
        self.discard_pile.push(card);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_deck() {
        let mut deck = PolicyDeck::new();
        {
            let all_cards = (0..17).map(|_| deck.draw().unwrap());
            assert_eq!(all_cards.filter(Policy::is_liberal).count(), 11);
        }
        assert!(deck.draw().is_none());
    }

    #[test]
    fn test_policy_deck_with_discard() {
        let mut deck = PolicyDeck::new();
        let mut liberal_count = 0;
        let mut fascist_count = 0;

        for _ in 0..15 {
            match deck.draw() {
                Some(Policy::Liberal) => liberal_count += 1,
                Some(Policy::Fascist) => fascist_count += 1,
                None => (),
            }
            let card = deck.draw().unwrap();
            deck.discard(card);
            let card = deck.draw().unwrap();
            deck.discard(card);
        }

        for _ in 0..2 {
            match deck.draw() {
                Some(Policy::Liberal) => liberal_count += 1,
                Some(Policy::Fascist) => fascist_count += 1,
                None => (),
            }
        }

        assert_eq!(liberal_count, 11);
        assert_eq!(fascist_count, 6);
    }
}
