use trotter::{Actor, Titan, UserAgent};

pub async fn mk_req(mut url: String) -> anyhow::Result<String> {
    if !url.ends_with("/") {
        url = format!("{}/", url);
    }

    let requester = Actor::default().user_agent(UserAgent::Webproxy);

    let response = requester.get(url).await?.gemtext()?;

    Ok(response)
}

pub async fn mk_titan(mut url: String) -> anyhow::Result<String> {
    Ok("titan_test".to_string())
}

