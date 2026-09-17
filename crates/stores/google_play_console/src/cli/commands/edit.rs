use crate::cli::GlobalArgs;
use crate::{GooglePlayContext, print_json, print_table};
use anyhow::Result;
use clap::{Args, Subcommand};
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Args, Debug)]
pub struct EditArgs {
    #[command(subcommand)]
    pub command: EditCommand,
}

#[derive(Subcommand, Debug)]
pub enum EditCommand {
    Create(EditPackageArgs),
    Commit(EditIdArgs),
    Delete(EditIdArgs),
}

#[derive(Args, Debug)]
pub struct EditPackageArgs {
    #[arg(long = "package-name")]
    pub package_name: String,
}

#[derive(Args, Debug)]
pub struct EditIdArgs {
    #[arg(long = "package-name")]
    pub package_name: String,
    #[arg(long = "edit-id")]
    pub edit_id: String,
}

#[derive(Serialize)]
pub struct EditRow {
    #[serde(rename = "editId")]
    pub edit_id: String,
    #[serde(rename = "packageName")]
    pub package_name: String,
}

pub async fn execute(args: &EditArgs, global: &GlobalArgs) -> Result<()> {
    let ctx = GooglePlayContext::from_env().await?;
    match &args.command {
        EditCommand::Create(args) => {
            let edit_id = create_edit(&ctx, &args.package_name).await?;
            print_edit(
                EditRow {
                    edit_id,
                    package_name: args.package_name.clone(),
                },
                global,
            )
        }
        EditCommand::Commit(args) => {
            commit_edit(&ctx, &args.package_name, &args.edit_id).await?;
            print_edit(
                EditRow {
                    edit_id: args.edit_id.clone(),
                    package_name: args.package_name.clone(),
                },
                global,
            )
        }
        EditCommand::Delete(args) => {
            delete_edit(&ctx, &args.package_name, &args.edit_id).await?;
            print_edit(
                EditRow {
                    edit_id: args.edit_id.clone(),
                    package_name: args.package_name.clone(),
                },
                global,
            )
        }
    }
}

pub async fn create_edit(ctx: &GooglePlayContext, package_name: &str) -> Result<String> {
    let response: Value = ctx
        .http
        .post(ctx.api_url(&format!("/applications/{package_name}/edits")))
        .json(&json!({}))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(response
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string())
}

pub async fn commit_edit(ctx: &GooglePlayContext, package_name: &str, edit_id: &str) -> Result<()> {
    ctx.http
        .post(ctx.api_url(&format!(
            "/applications/{package_name}/edits/{edit_id}:commit"
        )))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

pub async fn delete_edit(ctx: &GooglePlayContext, package_name: &str, edit_id: &str) -> Result<()> {
    ctx.http
        .delete(ctx.api_url(&format!("/applications/{package_name}/edits/{edit_id}")))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

fn print_edit(row: EditRow, global: &GlobalArgs) -> Result<()> {
    if global.json.is_some() {
        return print_json(&row, global.json.as_deref());
    }
    print_table(
        &["PACKAGE_NAME", "EDIT_ID"],
        &[vec![row.package_name, row.edit_id]],
    );
    Ok(())
}
