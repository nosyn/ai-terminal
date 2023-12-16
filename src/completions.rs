use std::error::Error;
use std::io::{stdout, Write};

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::Client;

use futures::StreamExt;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct OpenAIClient {
    pub client: Client<OpenAIConfig>,
    pub conversation: Vec<ChatCompletionRequestMessage>,
}

impl OpenAIClient {
    pub fn new() -> Client<OpenAIConfig> {
        Client::new()
    }
    #[tokio::main]
    pub async fn chat(&mut self, message: &str) -> MyResult<()> {
        self.conversation.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(message)
                .build()?
                .into(),
        );

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo")
            .max_tokens(512u16)
            .messages(self.conversation.clone())
            .build()?;

        let mut stream = self.client.chat().create_stream(request).await?;

        // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
        //
        //  Note that stdout is frequently line-buffered by default so it may be necessary
        //  to use io::stdout().flush() to ensure the output is emitted immediately.
        //
        //  The print! macro will lock the standard output on each call.
        //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
        //  To avoid this, lock stdout with io::stdout().lock():

        let mut ai_response = Vec::new();
        let mut lock = stdout().lock();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    response.choices.iter().for_each(|chat_choice| {
                        if let Some(ref content) = chat_choice.delta.content {
                            ai_response.push(content.clone());
                            write!(lock, "{}", content).unwrap();
                        }
                    });
                }
                Err(err) => {
                    writeln!(lock, "error: {err}").unwrap();
                }
            }
            stdout().flush()?;
        }

        self.conversation.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(ai_response.join(" "))
                .build()?
                .into(),
        );

        Ok(())
    }
}
