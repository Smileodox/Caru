use rusqlite::{Connection, Result};

// Create the necessary tables for decks and flashcards
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deck (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS flashcard (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            deck_id INTEGER,
            question TEXT NOT NULL,
            answer TEXT NOT NULL,
            ease_factor REAL,
            FOREIGN KEY(deck_id) REFERENCES deck(id)
        )",
        [],
    )?;
    Ok(())
}
