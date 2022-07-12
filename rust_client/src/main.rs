mod cli;
mod http_requests;
use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use http_requests::make_requests;

// main chali main chali
// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    make_requests().await;

    Ok(())
}
