use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

use crate::{requests, gemtext_parse};

pub async fn draw_ui(mut content: String, mut url: String) -> anyhow::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut links: Vec<String>;
    (content, links) = gemtext_parse::gemtext_restructer(content, url.clone()).await?;

    let mut backlink = url.clone();

    let mut new_url = String::new();

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            let browser_view = Paragraph::new(content.clone())
                .wrap(Wrap { trim: true })
                .style(Style::new().white())
                .block(
                    Block::new()
                        .title(url.clone())
                        .title_style(Style::new().white().bold())
                        .borders(Borders::ALL)
                        .border_style(Style::new().white()),
                );

            frame.render_widget(browser_view, area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }

                        KeyCode::Char('r') => {
                            content = requests::mk_req(url.clone()).await?;
                            (content, links) =
                                gemtext_parse::gemtext_restructer(content, url.clone()).await?;
                        }

                        KeyCode::Char('n') => { 
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
                            content = requests::mk_req(url.clone()).await?;
                            (content, links) =
                                gemtext_parse::gemtext_restructer(content, url.clone()).await?;
                        }

                        KeyCode::Char('g') => {
                            let mut nonusize_link_address = String::new();
                            while let Event::Key(KeyEvent { code, .. }) = event::read()? {
                                match code {
                                    KeyCode::Enter => {
                                        break;
                                    }
                                    KeyCode::Char(c) => {
                                        nonusize_link_address.push(c);
                                    }
                                    _ => {}
                                }
                            }

                            let u32_link_address: u32 =
                                nonusize_link_address.parse().expect("Cannot convert");

                            let link_address: usize = u32_link_address as usize;

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
                                backlink = url.to_string();
                                url = link.to_string();
                                content = requests::mk_req(link.clone()).await?;
                            }

                            (content, links) =
                                gemtext_parse::gemtext_restructer(content, url.clone()).await?;
                        }

                        KeyCode::Char('b') => {
                            content = requests::mk_req(backlink.clone()).await?;
                            (content, links) =
                                gemtext_parse::gemtext_restructer(content, url.clone()).await?;
                        }

                        KeyCode::Char('t') => {}

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
