mod tests;
pub use crate::tests::example;

use cursive::Cursive;
use cursive::views::Dialog;
use cursive::views::TextView; 

fn main() {
    println!("Loading Technocrat...");
    cursive_window();
}


fn cursive_window()
{
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    
    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Welcome to Technocrat!"))
                         .title("Technocrat")
                         .button("Next", todo));

    // Starts the event loop.
    siv.run();
}

fn todo(s: &mut Cursive)
{
    s.pop_layer();
    s.add_layer(Dialog::text("Did you do the thing?")
        .title("Question 1")
        .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
        .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
        .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))));
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Results")
        .button("Finish", |s| s.quit()));
}


// EXAMPLE - TEST CASE FUNCTIONS

pub fn add_both_numbers(x: i8, y: i8) -> i8
{
    return x + y;
}

pub fn sub_both_numbers(x: i8, y: i8) -> i8 
{
    return x - y;
}
