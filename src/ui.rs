use std::{io::stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

pub async fn mk_req(url: String) -> anyhow::Result<String> {
    let response = trotter::trot(url.clone()).await?.gemtext()?;
    Ok(response)
}

pub async fn draw_ui(mut content: String, url: String) -> anyhow::Result<()> {
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
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('r') {
                    content = mk_req(url.clone()).await?
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('n') {
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

                    content = mk_req(new_url.clone()).await?;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
