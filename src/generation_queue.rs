use async_openai::Client;
use async_openai::types::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use sqlx::PgPool;
use tokio_util::sync::CancellationToken;
use tracing::log::{log, Level};
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
            // generate examples with openai
            // generate examples with openai
            if let Err(e) = generate_examples_for_word(&word_id).await {
                log!(Level::Error, "Error generating examples for word {}: {:?}", word_id.id, e);
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn generate_examples_for_word(word: &Word) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages(vec![
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a language learning app. You have a user who wants to learn a new word. The user gives you the word and you need to generate example sentences for the word.")
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(format!("Generate example sentences for the word {}, the translation that I'm using for it is \"{}\"", word.word, word.translation))
                .build()
                .unwrap()
                .into()
        ])
        .build()
        .unwrap();

    let response = client.chat().create(request).await?;

    if let Some(choice) = response.choices.first() {
        // Here you would save the generated examples to your database
        println!("Generated examples for word {}: {}", word.word, choice.clone().message.content.unwrap());
    }

    Ok(())
}