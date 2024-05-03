
#[macro_export]
macro_rules! print_message {
    ($message:expr, $position:expr) => {{
        let mut stdout = std::io::stdout();

        // Move the cursor to the desired position
        stdout
            .queue(cursor::MoveTo($position.0, $position.1))
            .unwrap();

        // Print the message
        stdout.queue(Print($message.as_bytes())).unwrap();

        // Flush the output to the terminal
        stdout.flush().unwrap();
    }};
}