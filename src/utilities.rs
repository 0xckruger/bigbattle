use std::fmt::Display;
use std::io::Write;
use crossterm::{cursor, QueueableCommand, terminal};
use crossterm::style::Print;

pub fn print_message(message: impl Display, position: (u16, u16)) {
    let mut stdout = std::io::stdout();

    // Move to desired position
    stdout.queue(cursor::MoveTo(position.0, position.1)).unwrap();

    // Get message lengths
    let max_message_length = 160;

    // Move cursor back by length of longer message
    stdout.queue(cursor::MoveLeft(max_message_length as u16)).unwrap();

    // Clear the line
    stdout.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();

    // Print the message
    stdout.queue(Print(format!("{}", message))).unwrap();

    // Flush the output to the terminal
    stdout.flush().unwrap();
}