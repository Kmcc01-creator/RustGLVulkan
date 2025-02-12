
// src/diff_analysis/chunking.rs
pub struct ChunkingStrategy {
    chunk_size: usize,
    overlap: usize,
}

impl ChunkingStrategy {
    pub fn new(chunk_size: usize, overlap: usize) -> Self {
        Self { chunk_size, overlap }
    }

    pub fn chunk_diff(&self, content: &str) -> Vec<DiffChunk> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for window in lines.windows(self.chunk_size) {
            let context = self.extract_context(window);
            chunks.push(DiffChunk {
                content: window.join("\n"),
                context,
                metadata: self.analyze_chunk(window),
            });
        }
        
        chunks
    }
}