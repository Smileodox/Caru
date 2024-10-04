use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ListBox, Label, Orientation, DialogFlags, Entry, Box as GtkBox, MessageDialog, MessageType};
use gio::prelude::*;
use std::env;
use std::rc::Rc;
use std::cell::RefCell;

use crate::db::queries::{get_decks, insert_flashcard, get_deck_flashcards}; // Add the necessary imports
use rusqlite::Connection;
use crate::deck::flashcard::Flashcard; // Add Flashcard import

pub fn start() {
    let app = Application::new(
        Some("com.example.anki_clone"),
        Default::default(),
    )
    .expect("Failed to initialize GTK application");

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Anki Clone");
        window.set_default_size(600, 400);
        window.set_position(gtk::WindowPosition::Center);

        let vbox = gtk::Box::new(Orientation::Vertical, 10);
        
        // Initialize the SQLite connection and wrap it in Rc<RefCell<Connection>>
        let conn = Rc::new(RefCell::new(Connection::open("anki_clone.db").expect("Failed to open database")));

        // Create a ListBox to display decks
        let deck_list = ListBox::new();
        let conn_ref = conn.clone(); // Clone Rc to use in closure
        let decks = get_decks(&conn_ref.borrow()).expect("Failed to retrieve decks");
        for deck in decks {
            let label = Label::new(Some(&deck));
            deck_list.add(&label);
        }
        
        // Add a button for adding new flashcards
        let add_flashcard_button = Button::with_label("Add Flashcard");
        let conn_ref = conn.clone(); // Clone Rc for closure
        add_flashcard_button.connect_clicked(move |_| {
            // Open a dialog for adding a new flashcard
            let dialog = gtk::Dialog::with_buttons(
                Some("Add Flashcard"),
                None::<&ApplicationWindow>,
                DialogFlags::MODAL,
                &[("Add", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
            );
            
            let content_area = dialog.get_content_area(); // Use get_content_area instead of content_area()
            let question_entry = Entry::new();
            question_entry.set_placeholder_text(Some("Enter the question")); // Wrap in Some()
            let answer_entry = Entry::new();
            answer_entry.set_placeholder_text(Some("Enter the answer")); // Wrap in Some()

            let box_layout = GtkBox::new(Orientation::Vertical, 10);
            box_layout.pack_start(&question_entry, true, true, 0);
            box_layout.pack_start(&answer_entry, true, true, 0);
            content_area.add(&box_layout);

            dialog.show_all();

            // When the user clicks "Add", insert the flashcard into the database
            let conn_clone = conn_ref.clone(); // Clone Rc for the callback
            dialog.connect_response(move |dialog, response| {
                if response == gtk::ResponseType::Ok {
                    let question = question_entry.get_text().to_string(); // Use get_text()
                    let answer = answer_entry.get_text().to_string(); // Use get_text()

                    if !question.is_empty() && !answer.is_empty() {
                        let flashcard = Flashcard::new(question, answer);
                        let deck_id = 1; // Assuming the first deck for now, you can improve this to use the selected deck
                        insert_flashcard(&conn_clone.borrow(), deck_id, &flashcard)
                            .expect("Failed to insert flashcard into the database");
                        println!("Flashcard added: {}", flashcard.question);
                    }
                }
                dialog.close();
            });
        });

        // Add a button for reviewing flashcards
        let review_button = Button::with_label("Review Flashcards");
        let conn_ref = conn.clone(); // Clone Rc for closure
        review_button.connect_clicked(move |_| {
            let deck_id = 1; // Select the first deck for now
            let flashcards = get_deck_flashcards(&conn_ref.borrow(), deck_id).expect("Failed to retrieve flashcards");
            
            for card in flashcards {
                // Show a dialog with the flashcard's question
                let question_dialog = MessageDialog::new(
                    None::<&ApplicationWindow>,
                    gtk::DialogFlags::MODAL,
                    MessageType::Info,
                    gtk::ButtonsType::Ok,
                    &format!("Question: {}", card.question),
                );
                question_dialog.run();
                question_dialog.close();

                // After showing the question, show the answer
                let answer_dialog = MessageDialog::new(
                    None::<&ApplicationWindow>,
                    gtk::DialogFlags::MODAL,
                    MessageType::Info,
                    gtk::ButtonsType::Ok,
                    &format!("Answer: {}", card.answer),
                );
                answer_dialog.run();
                answer_dialog.close();
            }
        });

        vbox.pack_start(&deck_list, true, true, 0);
        vbox.pack_start(&add_flashcard_button, false, false, 10);
        vbox.pack_start(&review_button, false, false, 10);

        window.add(&vbox);
        window.show_all();
    });

    app.run(&env::args().collect::<Vec<_>>());
}
