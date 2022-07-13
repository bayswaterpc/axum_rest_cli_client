pub mod spotify_example;

use spotify_example::spotify_request;
use futures::join;
use reqwest::{self, Result};
use std::collections::HashMap;


async fn reqwest_readme_example() -> Result<HashMap<String, String>> {
    // chaining .await will yield our query result
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(resp)
}

async fn reade_me_printer() {
    println!("{:#?}", reqwest_readme_example().await);
}


pub async fn make_requests() {
    let fut1 = spotify_request();
    let fut2 = reade_me_printer();

    join!(fut1, fut2);
}
