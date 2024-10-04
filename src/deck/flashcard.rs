#[derive(Debug, Clone)]
pub struct Flashcard {
    pub id: Option<i32>,
    pub question: String,
    pub answer: String,
    pub ease_factor: f64,
}

impl Flashcard {
    pub fn new(question: String, answer: String) -> Flashcard {
        Flashcard {
            id: None,
            question,
            answer,
            ease_factor: 2.5,
        }
    }
}
