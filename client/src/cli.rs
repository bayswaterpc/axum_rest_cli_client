use crate::requests::todos::{delete_todo, get_root, get_todo, patch_todo, post_todo};
use anyhow::{bail, Context, Ok, Result};
use clap::Parser;
use common::{Pagination, PostTodo, UpdateTodo, TODOS_PATH};
use std::ffi::OsString;
use std::io;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Request {
    Post,
    Put,
    Patch,
    Get,
    Delete,
    HelloWorld,
}

#[derive(Parser, Debug)]
pub struct Args {
    /// enum for supported http request
    #[clap(value_enum, long, short, value_parser, default_value_t=Request::Get)]
    request: Request,

    /// Uri for request
    #[clap(long, value_parser,  short, value_parser, default_value_t=TODOS_PATH.to_string())]
    uri: String,

    /// Id used in patch & delete commands
    #[clap(long, value_parser, short, value_parser)]
    id: Option<String>,

    /// json file with data.
    #[clap(long, short, value_parser)]
    file: Option<String>,

    /// set to true to quit
    #[clap(long, short, value_parser, default_value_t = false)]
    quit: bool,
}

// see this for parsing from string....

fn string_to_args(string: &str) -> Vec<OsString> {
    // TODO: add handling of whitespace characters in quotes and character escaping
    let mut args = vec![OsString::from("client")];
    for arg in string.split_whitespace() {
        args.push(arg.into());
    }
    args
}

pub fn read_commands() -> Result<Args> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let strings = string_to_args(&buffer);
    let args = Args::try_parse_from(strings.iter())?;
    //  print!("read_commands args {:?}", args);
    Ok(args)
}

pub async fn run_cli() -> Result<()> {
    println!("Enter REST request, run -h for help");
    let mut args = read_commands()?;
    while !args.quit {
        cli_execute(args)
            .await
            .with_context(|| "command execution error".to_string())?;

        println!("Run another command, enter '-q' to quit");
        args = read_commands()?;
    }
    Ok(())
}

pub async fn cli_execute(args: Args) -> Result<()> {
    match args.request {
        Request::HelloWorld => {
            let resp = get_root().await?;
            println!("{:?} Response: {:?}", args.request, resp);
        }
        Request::Delete => {
            if args.id.is_none() {
                bail!("Id required for delete");
            }
            let resp = delete_todo(&args.id.unwrap()).await;
            println!("{:?} Response: {:?}", args.request, resp);
        }
        Request::Get => {
            let pagination = if args.file.is_none() {
                Pagination {
                    offset: None,
                    limit: None,
                }
            } else {
                let file_contents = std::fs::read_to_string(args.file.unwrap())?;
                let pg: Pagination = serde_json::from_str(file_contents.as_str())?;
                pg
            };
            let resp = get_todo(pagination).await;
            println!("{:?} Response: {:?}", args.request, resp);
        }
        _ => {
            if args.file.is_none() {
                bail!(format!("Input json file required for {:?}", args.request));
            }
            let file_contents = std::fs::read_to_string(args.file.unwrap())?;

            match args.request {
                Request::Post => {
                    let todo: PostTodo = serde_json::from_str(file_contents.as_str())?;
                    let resp = post_todo(todo).await;
                    println!("{:?} Response: {:?}", args.request, resp);
                }
                Request::Put => {}
                Request::Patch => {
                    if args.id.is_none() {
                        bail!("Id required for patch");
                    }
                    let todo: UpdateTodo = serde_json::from_str(file_contents.as_str())?;
                    let resp = patch_todo(&args.id.unwrap(), todo).await;
                    println!("{:?} Response: {:?}", args.request, resp);
                }
                _ => panic!("unreachable"),
            }
        }
    }
    Ok(())
}
