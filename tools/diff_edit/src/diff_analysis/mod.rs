// src/diff_analysis/mod.rs
pub mod semantic;
pub mod llm;
pub mod ast;
pub mod chunking;

use std::error::Error;
use similar::{Algorithm, ChangeTag, DiffOp};

// Core traits for different diff strategies
pub trait DiffStrategy {
    fn compare(&self, old: &str, new: &str) -> Result<DiffAnalysis, Box<dyn Error>>;
}

pub struct DiffAnalysis {
    pub changes: Vec<Change>,
    pub complexity: ChangeComplexity,
    pub semantic_groups: Vec<ChangeGroup>,
}

#[derive(Debug)]
pub struct Change {
    pub kind: ChangeKind,
    pub content: String,
    pub context: ChangeContext,
}

#[derive(Debug)]
pub enum ChangeKind {
    Rename { old: String, new: String },
    Move { from: usize, to: usize },
    Edit { similarity: f32 },
    Addition,
    Deletion,
    Structural,  // For AST-level changes
}

#[derive(Debug)]
pub struct ChangeContext {
    pub scope: String,      // Function/class/module
    pub imports: Vec<String>,
    pub dependencies: Vec<String>,
}
