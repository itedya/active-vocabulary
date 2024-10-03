use async_openai::Client;
use async_openai::types::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use sqlx::PgPool;
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::log::{log, Level};
use tracing::log::__private_api::log;
use crate::gpt::Chat;
use crate::models::Word;

pub async fn generation_queue_worker(cancellation_token: CancellationToken, pool: PgPool) {
    while !cancellation_token.is_cancelled() {
        let word_ids: Result<Vec<Word>, sqlx::Error> = sqlx::query_as("SELECT words.* FROM example_generation_queue INNER JOIN words ON words.id = word_id")
            .fetch_all(&pool)
            .await;

        let word_ids = match word_ids {
            Ok(v) => v,
            Err(e) => {
                log!(Level::Error, "Error fetching words from example_generation_queue: {:?}", e);
                continue;
            }
        };

        for word_id in word_ids {
            let (example, translation) = match generate_examples_for_word(&word_id).await {
                Ok(v) => v,
                Err(e) => {
                    log!(Level::Error, "Error generating examples for word {}: {:?}", word_id.id, e);
                    continue;
                }
            };

            let mut tx = match pool.begin().await {
                Ok(v) => v,
                Err(e) => {
                    log!(Level::Error, "Error starting transaction: {:?}", e);
                    continue;
                }
            };

            let result = sqlx::query("INSERT INTO examples (word_id, example, translation) VALUES ($1, $2, $3)")
                .bind(word_id.id)
                .bind(example)
                .bind(translation)
                .execute(&mut *tx)
                .await;

            if let Err(e) = result {
                log!(Level::Error, "Error inserting example for word {}: {:?}", word_id.id, e);
                continue;
            }

            let result = sqlx::query("DELETE FROM example_generation_queue WHERE word_id = $1")
                .bind(word_id.id)
                .execute(&mut *tx)
                .await;

            if let Err(e) = result {
                log!(Level::Error, "Error deleting word from example_generation_queue: {:?}", e);
                continue;
            }

            if let Err(e) = tx.commit().await {
                log!(Level::Error, "Error committing transaction: {:?}", e);
                continue;
            }

            log!(Level::Info, "Generated example for word {}", word_id.id);
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[derive(Debug, Error)]
enum ExampleGenerationError {
    #[error("Error processing chat: {0}")]
    ProcessChatError(#[from] crate::gpt::ProcessChatError),

    #[error("Invalid response from llm, expected 2 lines, got {0}")]
    InvalidResponse(usize),
}

async fn generate_examples_for_word(word: &Word) -> Result<(String, String), ExampleGenerationError> {
    let client = Client::new();

    let prompt = concat!(
    "You are a language learning app. ",
    "You have a user who wants to learn a new word. ",
    "The user gives you the word and you need to generate example sentence for the word.",
    "The user also gives you the translation that they're using for the word.",
    "Respond with only the ONE example sentence and its translation. No need to include any additional information.",
    "\r\n",
    "\r\n",
    "<example>",
    "<user>apple\r\njabłko</user>",
    "<your-response>I ate an apple yesterday.\r\nZjadłem jabłko wczoraj.</your-response>",
    "</example>"
    );

    let mut chat = Chat::new();

    chat.add_system_message(prompt);
    chat.add_user_message(&format!("{}\r\n{}", word.word, word.translation));

    let response = chat.process(client).await
        .map_err(ExampleGenerationError::ProcessChatError)?;

    let splitted_response = response.split("\n").map(|part| part.to_string()).collect::<Vec<String>>();

    if splitted_response.len() != 2 {
        return Err(ExampleGenerationError::InvalidResponse(splitted_response.len()));
    }

    let example = splitted_response[0].clone();
    let translation = splitted_response[1].clone();

    Ok((example, translation))
}