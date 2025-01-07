//! Demonstrates how to block read characters or a full line.
//! Just note that crossterm is not required to do this and can be done with `io::stdin()`.
//!
//! cargo run --example event-read-char-line

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    println!("read line:");
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Enter => {
                println!("{:}", line);
                line = String::from("");
            }
            KeyCode::Char(c) => {
                line.push(c);
            }
            _ => {}
        }
    }

    Ok(line)
}

fn main() {
    let _ = read_line();
}
