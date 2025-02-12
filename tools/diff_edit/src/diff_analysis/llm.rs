
// src/diff_analysis/llm.rs
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LLMRequest {
    pub diff_content: String,
    pub context: ChangeContext,
    pub analysis_type: LLMAnalysisType,
}

#[derive(Deserialize)]
pub struct LLMResponse {
    pub suggested_changes: Vec<SuggestedChange>,
    pub impact_analysis: ImpactAnalysis,
    pub review_comments: Vec<ReviewComment>,
}

#[derive(Debug)]
pub enum LLMAnalysisType {
    CodeReview,
    RefactoringProposal,
    SecurityAudit,
    PerformanceImpact,
}

pub struct LLMDiffAnalyzer {
    api_client: Box<dyn LLMClient>,
    chunk_strategy: ChunkingStrategy,
}

#[async_trait]
pub trait LLMClient {
    async fn analyze_diff(&self, request: LLMRequest) -> Result<LLMResponse, Box<dyn Error>>;
}

impl LLMDiffAnalyzer {
    pub fn new(api_client: Box<dyn LLMClient>) -> Self {
        Self {
            api_client,
            chunk_strategy: ChunkingStrategy::new(50, 10),
        }
    }

    pub async fn analyze(&self, diff: &str, context: ChangeContext) -> Result<LLMResponse, Box<dyn Error>> {
        // Break diff into semantic chunks
        let chunks = self.chunk_strategy.chunk_diff(diff);
        
        // Prepare LLM request with chunked content
        let request = LLMRequest {
            diff_content: self.format_chunks(&chunks),
            context,
            analysis_type: LLMAnalysisType::CodeReview,
        };
        
        // Get LLM analysis
        self.api_client.analyze_diff(request).await
    }

    fn format_chunks(&self, chunks: &[DiffChunk]) -> String {
        // Format chunks for LLM consumption with appropriate context
        chunks.iter()
            .map(|chunk| format!("```\n{}\n```\nContext: {}\n", chunk.content, chunk.context))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/*
// Example implementation for OpenAI
pub struct OpenAIClient {
    api_key: String,
    model: String,
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn analyze_diff(&self, request: LLMRequest) -> Result<LLMResponse, Box<dyn Error>> {
        // Implementation for OpenAI API calls
        todo!()
    }
}

    */ 
