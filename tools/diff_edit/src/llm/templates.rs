// src/llm/templates.rs
use serde::{Serialize, Deserialize};
use tera::{Tera, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub template: String,
    pub parameters: Vec<String>,
    pub max_tokens: usize,
    pub temperature: f32,
    pub stop_sequences: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemPrompt {
    pub role: String,
    pub content: String,
}

pub const DEFAULT_DIFF_TEMPLATE: &str = r#"
You are a expert code reviewer analyzing the following code changes. Focus on:
1. Semantic changes and their implications
2. Potential bugs or issues
3. Performance implications
4. Security considerations
5. Architectural impact

Code Context:
Language: {{ language }}
File: {{ filename }}
Scope: {{ scope }}

Original Code:
```{{ language }}
{{ original_code }}
```

Modified Code:
```{{ language }}
{{ modified_code }}
```

Dependencies:
{% for dep in dependencies %}
- {{ dep }}
{% endfor %}

Please analyze these changes and provide:
1. A summary of the changes
2. Potential issues or concerns
3. Suggested improvements
4. Impact assessment

Format your response as JSON with the following structure:
{
    "summary": "Brief description of changes",
    "issues": [{"severity": "HIGH|MEDIUM|LOW", "description": "Issue description"}],
    "suggestions": ["Improvement suggestion 1", "Improvement suggestion 2"],
    "impact": {
        "breaking_changes": boolean,
        "performance": "POSITIVE|NEUTRAL|NEGATIVE",
        "security": "IMPROVED|NEUTRAL|COMPROMISED",
        "maintenance": "IMPROVED|NEUTRAL|DEGRADED"
    }
}
"#;
