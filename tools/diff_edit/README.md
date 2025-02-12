// README.md
# Intelligent Diff Tool

An advanced diff tool with LLM integration for intelligent code analysis.

## Features
- Git integration
- Side-by-side diff viewing
- LLM-powered code analysis
- Semantic diff parsing
- AST-based change detection
- Rate-limited API integration
- Multiple LLM provider support

## Usage
```bash
cargo run -- analyze path/to/repo
cargo run -- review path/to/file
cargo run -- batch path/to/files --llm
```

## Configuration
Create a `.env` file:
```env
OPENAI_API_KEY=your_key_here
LLM_RATE_LIMIT=50
LLM_WINDOW_MS=60000
```
