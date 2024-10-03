use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use thiserror::Error;

pub enum Role {
    User,
    Assistant,
    System,
}

pub struct ChatMessage {
    role: Role,
    content: String,
}

pub struct Chat {
    messages: Vec<ChatMessage>,
}

#[derive(Error, Debug)]
pub enum ProcessChatError {
    #[error("OpenAI error: {0}")]
    OpenAIError(#[from] OpenAIError),

    #[error("Model returned no response")]
    ModelReturnedNoResponse,

    #[error("Model returned response with empty content")]
    ModelReturnedResponseWithEmptyContent,

    #[error("Failed to build request: {0}")]
    FailedToBuildRequest(OpenAIError),
}


impl Chat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_system_message(&mut self, content: &str) {
        self.messages.push(ChatMessage {
            role: Role::System,
            content: content.to_string(),
        });
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.messages.push(ChatMessage {
            role: Role::User,
            content: content.to_string(),
        });
    }

    pub fn add_assistant_message(&mut self, content: &str) {
        self.messages.push(ChatMessage {
            role: Role::Assistant,
            content: content.to_string(),
        });
    }

    pub async fn process(&self, openai_client: async_openai::Client<OpenAIConfig>) -> Result<String, ProcessChatError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o-mini")
            .messages(self.messages.iter().map(|message| {
                match message.role {
                    Role::User => ChatCompletionRequestUserMessageArgs::default()
                        .content(message.content.clone())
                        .build()
                        .unwrap()
                        .into(),
                    Role::Assistant => ChatCompletionRequestAssistantMessageArgs::default()
                        .content(message.content.clone())
                        .build()
                        .unwrap()
                        .into(),
                    Role::System => ChatCompletionRequestSystemMessageArgs::default()
                        .content(message.content.clone())
                        .build()
                        .unwrap()
                        .into(),
                }
            }).collect::<Vec<ChatCompletionRequestMessage>>())
            .build()
            .map_err(ProcessChatError::FailedToBuildRequest)?;

        Ok(openai_client.chat().create(request).await
            .map_err(ProcessChatError::OpenAIError)?
            .choices
            .first()
            .ok_or(ProcessChatError::ModelReturnedNoResponse)?
            .clone()
            .message
            .content
            .ok_or(ProcessChatError::ModelReturnedResponseWithEmptyContent)?) // This is a bit ugly, but it's the shortest one I could come up with
    }
}