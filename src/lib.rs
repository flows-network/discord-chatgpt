use std::{env, str};
use dotenv::dotenv;
use discord_flows::{model::Message, Bot, ProvidedBot};
use flowsnet_platform_sdk::logger;
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use store_flows as store;
use serde_json::json;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    logger::init();

    let discord_token = env::var("discord_token").unwrap();
    let placeholder_text = env::var("placeholder").unwrap_or("Typing ...".to_string());
    let system_prompt = std::env::var("system_prompt").unwrap_or("You are a helpful assistant answering questions on Discord.".to_string());

    let bot = ProvidedBot::new(discord_token);
    bot.listen(|msg| handler(&bot, &placeholder_text, &system_prompt, msg)).await;
}

async fn handler(bot: &ProvidedBot, placeholder_text: &str, system_prompt: &str, msg: Message) {
    let discord = bot.get_client();
    if msg.author.bot {
        log::info!("ignored bot message");
        return;
    }
    let channel_id = msg.channel_id;
    let content = msg.content;


    if content.eq_ignore_ascii_case("/restart") {
        _ = discord.send_message(
            channel_id.into(),
            &serde_json::json!({
                "content": "Ok, I am starting a new conversation."
            }),
        ).await;
        store::set(&channel_id.to_string(), json!(true), None);
        log::info!("Restarted converstion for {}", channel_id);
        return;
    }

    let restart = store::get(&channel_id.to_string())
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if restart {
        log::info!("Detected restart = true");
        store::set(&channel_id.to_string(), json!(false), None);
    }

    let placeholder  = discord.send_message(
        channel_id.into(),
        &serde_json::json!({
            "content": placeholder_text
        }),
    ).await.unwrap();

    let mut openai = OpenAIFlows::new();
    openai.set_retry_times(3);
    let co = ChatOptions {
        // model: ChatModel::GPT4,
        model: ChatModel::GPT35Turbo,
        restart: restart,
        system_prompt: Some(system_prompt),
        ..Default::default()
    };

    match openai.chat_completion(&channel_id.to_string(), &content, &co).await {
        Ok(r) => {
            _ = discord.edit_message(
                channel_id.into(), placeholder.id.into(),
                &serde_json::json!({
                    "content": r.choice
                }),
            ).await;
        }
        Err(e) => {
            _ = discord.edit_message(
                channel_id.into(), placeholder.id.into(),
                &serde_json::json!({
                    "content": "Sorry, an error has occured. Please try again later!"
                }),
            ).await;
            log::error!("OpenAI returns error: {}", e);
        }
    }

}
