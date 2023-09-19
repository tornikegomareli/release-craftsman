use chat_gpt_lib_rs::{ChatGPTClient, ChatInput, Message, Model, Role};
use spinoff::{spinners, Color, Spinner, Streams};
use std::io::Result;

pub struct ChatGptRunner {}

impl ChatGptRunner {
    pub async fn run_chat_gpt(api_key: &str, model: Model, message: &str) -> Result<String> {
        let base_url = "https://api.openai.com";

        let client = ChatGPTClient::new(api_key, base_url);

        let chat_input = ChatInput {
            model,
            messages: vec![
                Message {
                    role: Role::System,
                    content: "You are a helpful assistant.".to_string(),
                },
                Message {
                    role: Role::User,
                    content: message.to_string(),
                },
            ],
            ..Default::default()
        };

        let mut spinner = Spinner::new(
            spinners::Moon,
            "👷🏻‍⚒️ Generating release notes... Please wait a while",
            Color::Red,
        );
        let response = client.chat(chat_input).await.unwrap();
        spinner.success("Release notes generated successfully");
        Ok(response.choices[0].message.content.clone())
    }
}
