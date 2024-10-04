use rusqlite::{params, Connection, Result};
use crate::deck::flashcard::Flashcard;

// Insert a flashcard into the database
pub fn insert_flashcard(conn: &Connection, deck_id: i32, card: &Flashcard) -> Result<()> {
    conn.execute(
        "INSERT INTO flashcard (deck_id, question, answer, ease_factor) VALUES (?1, ?2, ?3, ?4)",
        params![deck_id, card.question, card.answer, card.ease_factor],
    )?;
    Ok(())
}

// Retrieve all flashcards for a given deck
pub fn get_deck_flashcards(conn: &Connection, deck_id: i32) -> Result<Vec<Flashcard>> {
    let mut stmt = conn.prepare("SELECT id, question, answer, ease_factor FROM flashcard WHERE deck_id = ?1")?;
    let flashcard_iter = stmt.query_map([deck_id], |row| {
        Ok(Flashcard {
            id: row.get(0)?,
            question: row.get(1)?,
            answer: row.get(2)?,
            ease_factor: row.get(3)?,
        })
    })?;

    let mut flashcards = Vec::new();
    for flashcard in flashcard_iter {
        flashcards.push(flashcard?);
    }
    Ok(flashcards)
}

// Insert a deck into the database
pub fn insert_deck(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO deck (name) VALUES (?1)",
        params![name],
    )?;
    Ok(())
}

// Retrieve all decks from the database
pub fn get_decks(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM deck")?;
    let deck_iter = stmt.query_map([], |row| row.get(0))?;

    let mut decks = Vec::new();
    for deck in deck_iter {
        decks.push(deck?);
    }
    Ok(decks)
}
