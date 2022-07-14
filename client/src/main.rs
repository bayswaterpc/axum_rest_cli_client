mod cli;
mod requests;
use anyhow::{Context, Result};
use cli::run_cli;

#[tokio::main]
async fn main() -> Result<()> {
    run_cli()
        .await
        .with_context(|| "run_cli error".to_string())?;

    Ok(())
}
