use std::{
    fs::File,
    error::Error,
    io::{BufReader, BufRead}
};

use reqwest;

extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;

use crate::utils::{
    file::FileUtils, misc::Misc, validation::Validate
};

use crate::cmd::download::Download;

use crate::configs::global::Global;

use super::syntax::Lexico;

pub struct Paimon;

impl Paimon {

    pub fn header() {
        let standard_font = FIGfont::standard().unwrap();

        if let Some(title) = standard_font.convert(Global::APP_NAME) {
            println!("{}", title.to_string().blue());
            println!("-------------------------------------------------------------------");
            println!("📜 Version: {}", Global::APP_VERSION.yellow());
            println!("🏠 Homepage: {} | {}", Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
            println!("⏰ Started in: {}", Misc::date_time().blue());
            println!("-------------------------------------------------------------------");
        }
    }

    pub async fn read_local_file(file_path: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if let Err(e) = Validate::validate_file(file_path) {
            eprintln!("{}", e);
            return Err(Box::new(e));
        }
        
        let mut path = String::new();
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
    
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();

            if !Lexico::handle_check_macro_line(&trimmed_line, "open_link") {
                if path.is_empty() {
                    path = Lexico::handle_get_path(trimmed_line);
                    let _ = FileUtils::new_path(&path);
                }

                Download::download_file(
                    &trimmed_line,
                    &path,
                    no_ignore, 
                    no_comments, 
                    kindle.clone()
                ).await?;
            } else {
                let _ = Misc::open_url(trimmed_line);
            }
        }
    
        Ok(())
    }
    
    pub async fn read_remote_file(url: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        Validate::validate_file_type(url, ".txt")?;
    
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
    
        let reader = BufReader::new(&bytes[..]);
    
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed_line = line.trim();
            
            let path = Lexico::handle_get_path(trimmed_line);
            let _ = FileUtils::new_path(&path);

            Download::download_file(
                &trimmed_line,
                &path,
                no_ignore, 
                no_comments, 
                kindle.clone()
            ).await?;
        }
    
        Ok(())
    }
    
    pub async fn run(run: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if !run.starts_with("http") {
            if let Err(_) = Self::read_local_file(run, no_ignore, no_comments, kindle).await {}
        } else {
            if let Err(e) = Self::read_remote_file(run, no_ignore, no_comments, kindle).await {
                eprintln!("Error: {}", e);
            }
        }

        Ok(())
    }

}
