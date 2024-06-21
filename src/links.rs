use colored::*;
use regex::Regex;

pub async fn create_link(domain: String, sublink: String, url: String) -> anyhow::Result<String> {
    let mut formed_link: String = url;
    if !sublink.starts_with("/") && sublink.ends_with(".gmi") | sublink.ends_with(".txt") {
        formed_link = format!("{}{}/{}", "gemini://", domain, sublink);
    } else if sublink.starts_with("/") {
        formed_link = format!("{}{}{}", "gemini://", domain, sublink);
    } else if sublink.starts_with("gemini://") {
        formed_link = formed_link;
    }
    Ok(formed_link)
}

pub async fn get_path(url: String) -> anyhow::Result<String> {
    let result: String;

    let pattern: String;
    if url.ends_with(".gmi") {
        pattern = format!(
            r"{}(.*){}",
            regex::escape("gemini://"),
            regex::escape(".gmi")
        );
    } else {
        pattern = format!(r"{}(.*)", regex::escape("gemini://"));
    }

    let re = Regex::new(&pattern).unwrap();

    // Apply the regex pattern to the original string
    if let Some(captures) = re.captures(&url) {
        if let Some(matched) = captures.get(1) {
            result = matched.as_str().to_string();
        } else {
            println!("{}: No match found for the capture group.", "Error".red());
            std::process::exit(1);
        }
    } else {
        println!(
            "{}: Pattern not found in the original string.",
            "Error".red()
        );
        std::process::exit(1);
    }

    Ok(result)
}

pub async fn extract_links(
    mut anchor_links: Vec<String>,
    gem_body: String,
    url: String,
) -> anyhow::Result<Vec<String>> {
    let re = Regex::new(r"=>\s*(\S+)").unwrap();

    for cap in re.captures_iter(&gem_body) {
        let unprocessed_link = &cap[1];
        let domain = get_path(url.clone()).await?;

        let link = create_link(domain, unprocessed_link.to_string(), url.clone()).await?;
        anchor_links.push(link);
    }

    Ok(anchor_links)
}
