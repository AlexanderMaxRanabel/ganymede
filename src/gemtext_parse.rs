pub fn link_fixer(link: String, mut domain: String) -> String {
    let mut new_link: String = String::new();
    if link.starts_with("/") {
        new_link = format!("{}{}{}", "gemini://", domain, link);
    }
    return new_link;
}

pub fn gemtext_restructer(mut content: String, url: String) -> (String, Vec<String>) {
    let vectorized_content: Vec<&str> = content.lines().collect();
    let mut new_content: Vec<String> = Vec::new();
    let mut links: Vec<String> = Vec::new();

    let mut link_iterator: i64 = -1;
    for line in vectorized_content {
        let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        if let Some(token_start) = tokens.get(0) {
            if token_start == "=>" && tokens.len() > 1 {
                let mut link = tokens.get(1).cloned().unwrap_or_else(|| {
                    println!(
                        "{}: No link has been found in tokens: {:?}",
                        colored::Colorize::red("Error"),
                        tokens
                    );
                    std::process::exit(1);
                });

                if !link.starts_with("gemini://") { 
                    let option_domain = url
                        .split_once("gemini://")
                        .map(|(_, rest)| rest.to_string());
                    let domain = match option_domain {
                        Some(s) => s,
                        None => std::process::exit(1),
                    };
                    link = link_fixer(link, domain);
                }

                links.push(link);
                link_iterator += 1;
                let link_iteration = format!("[{}]", link_iterator);
                let mut modifiable_tokens = tokens.clone();

                modifiable_tokens.insert(0, link_iteration);
                let new_line = modifiable_tokens.join(" ");

                new_content.push(new_line);
            } else {
                new_content.push(line.to_string());
            }
        }
    }

    content = new_content.join("\n");

    (content, links)
}
