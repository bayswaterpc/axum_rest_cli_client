use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self, Error, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let res = sub_function().await;

    match res {
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
    let res = reade_me_example().await;
    println!("{:#?}", res.unwrap_or_default());
}

async fn sub_function() -> Result<Response, Error> {
    // chaining .await will yield our query result
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/search")
        .header(AUTHORIZATION, "Bearer BQAP-Mms-zZsJjMlz8hNp96qVtwOC548dwELqNvyqsdPTbB_huV8grThUiiqB68UmlDRaO-IKY9X7WrJsEx_zS7uE3amG5nNwaQprixA9j9QL5g261RnE1Cd_92aOl3NpkVYxcyFgPsJs45k8-wR2TfBjHbTyuqAT3MuCQU0ARul7T1dwD8wN8aEhu4Q4UBtIdQ")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        // confirm the request using send()
        .send()
        // the rest is the same!
        // .text()
        .await;
    response
}

async fn reade_me_example() -> Result<HashMap<String, String>, Error> {
    // chaining .await will yield our query result
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(resp)
}
