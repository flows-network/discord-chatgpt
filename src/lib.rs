use discord_flows::{create_text_message_in_channel, listen_to_channel, TextMessage};
use dotenv::dotenv;
use flowsnet_platform_sdk::write_error_log;
use openai_flows::{chat_completion, ChatModel, ChatOptions};
use std::env;

#[no_mangle]
pub fn run() {

    dotenv().ok();
    
    let guild_name: String = match env::var("server_name") {
        Err(_) => "myserver".to_string(),
        Ok(name) => name,
    };
    let channel_name: String = match env::var("channel_name") {
        Err(_) => "general".to_string(),
        Ok(name) => name,
    };

    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "jaykchen".to_string(),
        Ok(name) => name,
    };

    listen_to_channel(&guild_name, &channel_name, |sm| {
        let prompt = "You are a helpful assistant answering questions on Discord. If someone greets you without asking a question, you should simply respond \"Hello, I am your assistant on Discord, built by the Second State team. I am ready for your instructions now!\"";

        if !sm.author.bot && !sm.content.trim().is_empty() {
            let msg = sm.content;
            let co = ChatOptions {
                model: ChatModel::GPT35Turbo,
                restart: false,
                restarted_sentence: Some(prompt),
            };
            if let Some(r) =
                chat_completion(&openai_key_name, &format!("chat_id#{}", sm.author.username), &msg, &co)
            {
                create_text_message_in_channel(&guild_name, &channel_name, r.choice, Some(sm.id));
            }
        };
    });
}
