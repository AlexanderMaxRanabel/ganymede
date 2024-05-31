use crate::links;

pub async fn gemtext_restructer(mut content: String, url: String) -> anyhow::Result<(String, Vec<String>)> {
    let vectorized_content: Vec<&str> = content.lines().collect();
    let mut new_content: Vec<String> = Vec::new();
    let mut links: Vec<String> = Vec::new();

    let mut link_iterator: i64 = -1;
    for line in vectorized_content {
        let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        if let Some(token_start) = tokens.get(0) {
            if token_start == "=>" && tokens.len() > 1 {
                link_iterator += 1;
                let link_iteration = format!("[{}]", link_iterator);
                links = links::extract_links(links, content.clone(), url.clone()).await?;

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

    Ok((content, links))
}
