use crate::cli::ModelRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelResponse {
    pub provider: String,
    pub model: String,
    pub content: String,
}

pub trait Provider {
    fn name(&self) -> &str;
    fn execute(&self, request: &ModelRequest) -> Result<ModelResponse, String>;
}

pub struct DummyProvider;

impl Provider for DummyProvider {
    fn name(&self) -> &str {
        "dummy"
    }

    fn execute(&self, request: &ModelRequest) -> Result<ModelResponse, String> {
        // Count how many files are formatted in the system message context
        let file_count = if let Some(sys_msg) = request.messages.iter().find(|m| m.role == "system")
        {
            sys_msg.content.matches("=== FILE: ").count()
        } else {
            0
        };

        let user_prompt = request
            .messages
            .iter()
            .find(|m| m.role == "user")
            .map(|m| m.content.as_str())
            .unwrap_or("");

        let content = format!(
            "Mock LLM response from CompText Dummy Provider.\n\
             Received prompt: \"{user_prompt}\"\n\
             Workspace context analyzed successfully: {file_count} files included.\n\
             Dummy status: offline-test-provider ok."
        );

        Ok(ModelResponse {
            provider: "dummy".to_string(),
            model: "dummy-model".to_string(),
            content,
        })
    }
}
