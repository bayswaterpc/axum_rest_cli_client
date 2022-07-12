use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

pub async fn spotify_request() {
    dotenv().ok(); 
    let spotify_bearer_token = std::env::var("SPOTIFY_BEARER_TOKEN").expect("Need SPOTIFY_BEARER_TOKEN env var");
    let auothorization_str = format!("Bearer {}", spotify_bearer_token);
    // chaining .await will yield our query result
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/search")
        .header(AUTHORIZATION, auothorization_str.as_str())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        // confirm the request using send()
        .send()
        // the rest is the same!
        // .text()
        .await;

    match response {
        Ok(response) => {
            match response.status() {
                reqwest::StatusCode::OK => {
                    // on success, parse our JSON to an APIResponse
                    match response.json::<APIResponse>().await {
                        Ok(parsed) => println!("Success! {:?}", parsed),
                        Err(_) => println!("Hm, the response didn't match the shape we expected."),
                    };
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    println!("Need to grab a new token");
                }
                other => {
                    println!("Uh oh! Something unexpected happened: {:?}", other);
                }
            };
        }
        Err(err) => {
            println!("Sending error {:#?}", err);
        }
    }
}
