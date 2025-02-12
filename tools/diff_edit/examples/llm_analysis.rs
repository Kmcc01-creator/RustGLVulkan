
// examples/llm_analysis.rs
use intelligent_diff::{DiffManager, LLMAnalyzer, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let diff_manager = DiffManager::new(config.clone())?;
    let llm_analyzer = LLMAnalyzer::new(config.llm)?;
    
    for diff in diff_manager.get_diffs()? {
        let analysis = llm_analyzer.analyze(&diff).await?;
        println!("Analysis: {:?}", analysis);
    }
    
    Ok(())
}