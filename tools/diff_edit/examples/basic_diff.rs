
// examples/basic_diff.rs
use intelligent_diff::{DiffManager, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let diff_manager = DiffManager::new(config)?;
    
    diff_manager.process_diffs()?;
    
    Ok(())
}
