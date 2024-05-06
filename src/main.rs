extern crate colored;

mod ui;
mod cmd;
mod utils;
mod addons;
mod configs;
mod args_cli;

use colored::*;
use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::Flags,
    configs::env::Env,
    cmd::paimon::Paimon,
    addons::scrape::Scrape
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = Env::download_env_file(false).await {
        eprintln!("Error: {}", err.to_string().red());
    }

    let flags = Flags::parse();
    let kindle_email = flags.kindle;
    let url = flags.url.as_deref().unwrap_or_default();
    let run = flags.run.as_deref().unwrap_or_default();
    let options = flags.options.as_deref().unwrap_or_default();

    Paimon::core(run, flags.noignore, flags.no_comments, kindle_email.to_owned()).await;
    
    Scrape::get(flags.scrape, url, flags.noignore, flags.no_comments).await?;

    Env::options_parser(options).await?;
    Ok(())
}
