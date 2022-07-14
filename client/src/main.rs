mod cli;
mod requests;
use anyhow::{Context, Result};
use cli::run_cli;
use common::{Pagination, PostTodo, UpdateTodo};
use requests::todos::{delete_todo, get_root, get_todo, patch_todo, post_todo};

#[tokio::main]
async fn main() -> Result<()> {
    run_cli()
        .await
        .with_context(|| "run_cli error".to_string())?;

    let root = get_root().await;
    println!("root {:?}", root);

    let post_params = PostTodo {
        text: "do this".to_string(),
    };
    let posted = post_todo(post_params).await;
    println!("posted{:?}", posted);
    let posted_id = posted
        .with_context(|| "posted error".to_string())?
        .id
        .to_string();

    let get_params = Pagination {
        limit: None,
        offset: None,
    };

    let got = get_todo(get_params)
        .await
        .with_context(|| "get error".to_string())?;
    println!("got {:#?}", got.len());

    let patch_params = UpdateTodo {
        text: Some("done".to_string()),
        completed: Some(true),
    };
    let patched = patch_todo(&posted_id, patch_params).await;
    println!("patched {:#?}", patched);

    let deleted_code = delete_todo(&posted_id).await;
    println!("deleted code A: {:#?}", deleted_code);

    let deleted_code = delete_todo(&posted_id).await;
    println!("deleted code B: {:#?}", deleted_code);

    let get_params = Pagination {
        limit: None,
        offset: None,
    };
    let got2 = get_todo(get_params)
        .await
        .with_context(|| "get error".to_string())?;
    println!("got {:#?}", got2.len());

    Ok(())
}
