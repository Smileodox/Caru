use super::flashcard::Flashcard;

#[derive(Debug)]
pub struct Deck {
    pub id: Option<i32>,
    pub name: String,
    pub cards: Vec<Flashcard>,
}

impl Deck {
    pub fn new(name: String) -> Deck {
        Deck {
            id: None,
            name,
            cards: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: Flashcard) {
        self.cards.push(card);
    }
}
