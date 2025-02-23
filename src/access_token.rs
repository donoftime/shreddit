use crate::cli::Config;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn new_access_token(args: &Config, client: &Client) -> String {
    let params = HashMap::from([
        ("grant_type", "password"),
        ("username", &args.username),
        ("password", &args.password),
    ]);

    let res: AccessTokenResponse = client
        .post("https://www.reddit.com/api/v1/access_token")
        .form(&params)
        .basic_auth(&args.client_id, Some(&args.client_secret))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    res.access_token
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}
