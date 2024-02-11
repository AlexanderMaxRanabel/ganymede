mod ui;
use std::env;

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

        let gem_res = ui::mk_req(url.clone()).await?;

        let draw_ui_handler = tokio::spawn(ui::draw_ui(gem_res.clone(), url.clone()));
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
