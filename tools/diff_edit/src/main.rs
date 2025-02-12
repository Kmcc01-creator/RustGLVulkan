// src/main.rs
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use colored::*;
use similar::{ChangeTag, TextDiff};
use walkdir::WalkDir;

mod git;
mod diff_display;
mod file_handler;

use crate::git::GitDiff;
use crate::diff_display::DiffDisplay;
use crate::file_handler::FileHandler;

fn main() -> Result<(), Box<dyn Error>> {
    let diff_manager = DiffManager::new()?;
    diff_manager.process_diffs()?;
    Ok(())
}

struct DiffManager {
    git_diff: GitDiff,
    diff_display: DiffDisplay,
    file_handler: FileHandler,
}

impl DiffManager {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            git_diff: GitDiff::new()?,
            diff_display: DiffDisplay::new(),
            file_handler: FileHandler::new(),
        })
    }

    fn process_diffs(&self) -> Result<(), Box<dyn Error>> {
        let diffs = self.git_diff.get_modified_files()?;
        
        for diff in diffs {
            println!("\nProcessing file: {}", diff.path.display());
            
            let old_content = self.file_handler.read_old_version(&diff)?;
            let new_content = fs::read_to_string(&diff.path)?;
            
            self.diff_display.show_side_by_side(&old_content, &new_content);
            
            if self.prompt_user_approval()? {
                self.file_handler.apply_changes(&diff, &new_content)?;
                println!("Changes applied successfully.");
            } else {
                println!("Changes skipped.");
            }
        }
        
        Ok(())
    }

    fn prompt_user_approval(&self) -> Result<bool, io::Error> {
        print!("Apply these changes? [y/N]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_lowercase() == "y")
    }
}


/*

// Example usage in main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize API client with configuration
    let config = LLMConfig {
        api_key: std::env::var("OPENAI_API_KEY")?,
        model: "gpt-4".to_string(),
        timeout: Duration::from_secs(60),
        max_retries: 3,
        batch_size: 4096,
        ..Default::default()
    };

    let api_client = LLMAPIClient::new(config)?;
    let rate_limiter = RateLimiter::new(50, 60000); // 50 requests per minute

    // Process diffs with rate limiting
    let diff_manager = DiffManager::new()?;
    for diff in diff_manager.git_diff.get_modified_files()? {
        let _guard = rate_limiter.acquire().await;
        
        let analysis = api_client
            .analyze_diff(&diff)
            .await?;

        println!("Analysis for {}:", diff.path.display());
        println!("Summary: {}", analysis.summary);
        // ... handle the rest of the analysis
    }

    Ok(())
}
    */ 