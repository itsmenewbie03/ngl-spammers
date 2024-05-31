use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub username: String,
    pub question: String,
    pub deviceId: String,
    pub gameSlug: Option<String>,
    pub referrer: String,
}

pub async fn send(target: &str, message: &str) -> bool {
    println!("Target:  {} | Message: {}", target, message);
    let url = "https://ngl.link/api/submit";
    let cli = Client::new();
    let body = RequestBody {
        username: target.to_owned(),
        question: message.to_owned(),
        deviceId: "69186a25-cedd-4829-a92d-ee7718219cf".to_owned(),
        gameSlug: None,
        referrer: "https://l.facebook.com/".to_owned(),
    };
    let res = cli.post(url).form(&body).send().await.unwrap();
    res.status().is_success()
}
