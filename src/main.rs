use std::io::{stdin /*, Write*/, stdout};

use crossterm::{
    /*event, */ execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{size, SetSize},
    tty::IsTty,
    ExecutableCommand,
};

// pub fn is_tty()
pub fn is_tty() {
    println!("size: {:?}", size().unwrap());
    execute!(stdout(), SetSize(100, 25)).unwrap();
    println!("resized: {:?}", size().unwrap());

    if stdin().is_tty() {
        println!("Is TTY");
    } else {
        println!("Is not TTY");
    }
}

fn main() -> std::io::Result<()> {
    is_tty();
    // using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("\nStyled text here."),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("\nStyled text here."))?
        .execute(ResetColor)?;

    Ok(())
}
