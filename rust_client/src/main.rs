mod cli;
mod requests;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use requests::make_requests;

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


// use clap::Parser;

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[clap(short, long, value_parser)]
//     name: String,

//     /// Number of times to greet
//     #[clap(short, long, value_parser, default_value_t = 1)]
//     count: u8,
// }

// fn main() {
//     let args = Args::parse();

//     for _ in 0..args.count {
//         println!("Hello {}!", args.name)
//     }
// }
