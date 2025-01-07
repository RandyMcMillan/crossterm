// add tokio main

use std::vec;

use std::io::{stdin /*, Write*/, stdout};

use crossterm::{
    /*event, */ execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{size, SetSize},
    tty::IsTty,
    ExecutableCommand,
};

use crossterm_examples::client::Client;
use crossterm_examples::req::ReqFilter;

use tungstenite::Message;

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

fn handle_message(relay_url: &String, message: &Message) -> Result<(), String> {
    println!("Received message from {}: {:?}", relay_url, message);

    println!("Events: {:?}", message);

    Ok(())
}
#[allow(dead_code)]
fn crossterm_execute() -> std::io::Result<()> {
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

#[tokio::main]
async fn main() {
    //let _ = crossterm_execute();

    let mut nostr_client = Client::new(vec!["wss://relay.damus.io"]).await.unwrap();

    // Run a new thread to handle messages

    println!("Listening...");
    let events = nostr_client.next_data().await.unwrap();
    print!("Events: {:?}", events);
    for (relay_url, message) in events.iter() {
        handle_message(relay_url, message).unwrap();
    }

    // Subscribe to my last text note
    let subscription_id = nostr_client
        .subscribe(vec![ReqFilter {
            ids: None,
            authors: None,
            kinds: Some(vec![0]),
            e: None,
            p: None,
            since: None,
            until: None,
            limit: Some(10),
        }])
        .await
        .unwrap();

    // Unsubscribe
    nostr_client.unsubscribe(&subscription_id).await.unwrap();
}
