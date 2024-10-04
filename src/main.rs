mod deck;
mod spaced_repetition;
mod db;
mod ui;

use rusqlite::Connection;
use deck::flashcard::Flashcard;
use db::schema::create_tables;

fn main() {
    let conn = Connection::open("anki_clone.db").expect("Failed to connect to database");

    create_tables(&conn).expect("Failed to create tables");

    ui::start();
}
