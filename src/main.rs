extern crate termion;

use colored::*;

use std::{
    env,
    io::{stdin, stdout, Write},
};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};
//use trotter::{Actor, UserAgent};

async fn key_events(url: String) -> anyhow::Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => {
                stdout.flush().unwrap();
                println!("{}", trotter::trot(url.clone()).await?.gemtext()?);
            },

            Key::Esc => break,
            _ => {
                println!("{}: Unknown operation", "Error".red());
            },
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url = args.get(1).cloned().unwrap_or_else(|| {
            println!("{}: No url has been provided", "Error".red());
            std::process::exit(1);
        });

        //let requester = Actor::default().user_agent(UserAgent::Indexer);
        //let actor_request = requester.get(url.clone()).await.unwrap();
        println!("{}", trotter::trot(url.clone()).await?.gemtext()?);

        let _key_task_handler = tokio::spawn(key_events(url.clone()));

        _key_task_handler.await??;
    } else {
        println!("{}: No Argument Nor Gemini URL provided", "Error".red());
    }

    Ok(())
}
