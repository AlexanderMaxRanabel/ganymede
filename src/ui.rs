use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

use crate::gemtext_parse;

use trotter::{Actor, UserAgent};

pub async fn mk_req(mut url: String) -> anyhow::Result<String> {
    if !url.ends_with("/") {
        url = format!("{}/", url);
    }

    let requester = Actor::default().user_agent(UserAgent::Webproxy);

    let response = requester.get(url).await?.gemtext()?;

    Ok(response)
}

pub async fn draw_ui(mut content: String, mut url: String) -> anyhow::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut links: Vec<String> = Vec::new();
    (content, links) = gemtext_parse::gemtext_restructer(content, url.clone());

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(content.clone()).white().on_black(), area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }

                        KeyCode::Char('r') => {
                            content = mk_req(url.clone()).await?;
                            (content, links) =
                                gemtext_parse::gemtext_restructer(content, url.clone());
                        }

                        KeyCode::Char('n') => {
                            let mut new_url = String::new();
                            while let Event::Key(KeyEvent { code, .. }) = event::read()? {
                                match code {
                                    KeyCode::Enter => {
                                        break;
                                    }
                                    KeyCode::Char(c) => {
                                        new_url.push(c);
                                    }
                                    _ => {}
                                }
                            }

                            url = new_url.chars().collect();
                            content = mk_req(url.clone()).await?;
                            (content, links) =
                                gemtext_parse::gemtext_restructer(content, url.clone());
                        }

                        KeyCode::Char('g') => {
                            let mut nonusize_link_address = String::new();
                            while let Event::Key(KeyEvent { code, .. }) = event::read()? {
                                match code {
                                    KeyCode::Enter => {
                                        break;
                                    }
                                    KeyCode::Char(c) => {
                                        nonusize_link_address = c.to_string();
                                    }
                                    _ => {}
                                }
                            }

                            let link_address: usize =
                                nonusize_link_address.parse().expect("Cannot convert");
                            let link = links.get(link_address).unwrap_or_else(|| {
                                content = "FATAL ERROR: Link from vec is unreachable".to_string();
                                std::process::exit(1);
                            });

                            if link.starts_with("https://") | link.starts_with("http://") {
                                match open::that(link.clone()) {
                                    Ok(()) => continue,
                                    Err(_err) => std::process::exit(1),
                                }
                            } else {
                                content = mk_req(link.clone()).await?;
                            }

                            (content, links) =
                                gemtext_parse::gemtext_restructer(content, url.clone());
                        }

                        _ => {
                            println!("{}: Unknown Operation", colored::Colorize::red("Error"));
                            break;
                        }
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
