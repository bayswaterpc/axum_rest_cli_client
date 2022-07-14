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

pub async fn reade_me_printer() {
    println!("{:#?}", reqwest_readme_example().await);
}
