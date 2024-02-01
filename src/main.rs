use std::{env, io::stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

async fn mk_req(url: String) -> anyhow::Result<String> {
    let response = trotter::trot(url.clone()).await?.gemtext()?;
    Ok(response)
}

async fn draw_ui(mut content: String, url: String) -> anyhow::Result<()> {
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
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url = args.get(1).cloned().unwrap_or_else(|| {
            println!(
                "{}: No url has been provided",
                colored::Colorize::red("Error")
            );
            std::process::exit(1);
        });

        let gem_res = mk_req(url.clone()).await?;

        let draw_ui_handler = tokio::spawn(draw_ui(gem_res.clone(), url.clone()));
        draw_ui_handler.await??;
    } else {
        println!(
            "{}: No Argument Nor Gemini URL provided",
            colored::Colorize::red("Error")
        );
        std::process::exit(1);
    }

    Ok(())
}
