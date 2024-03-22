use std::io::stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

pub async fn mk_req(url: String) -> anyhow::Result<String> {
    let response = trotter::trot(url.clone()).await?.gemtext()?;
    Ok(response)
}

pub async fn gemtext_parser(content: &str) -> Vec<&str> {
    let parsed_content: Vec<&str> = content.lines().collect();
}

pub async fn gemtext_restructurer(gemtext: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    for line in &gemtext {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0] == "=>" {
            
        }
    }
}

pub async fn draw_ui(mut content: String, mut url: String) -> anyhow::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

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
                        }

                        KeyCode::Char('g') => {
                            let parsed: Vec<&str> = gemtext_parser(content.as_str());
                            content = parsed.to_string();
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
