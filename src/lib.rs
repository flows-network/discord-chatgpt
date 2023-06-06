use discord_flows::{model::Message, Bot, DefaultBot};
use flowsnet_platform_sdk::logger;
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    logger::init();

    let channel_id = std::env::var("CHANNEL_ID").map(|c| c.parse().unwrap_or(0));

    let channel_id = match channel_id {
        Ok(c) if c != 0 => c,
        _ => {
            log::error!("channel_id not set");
            return;
        }
    };

    let bot = DefaultBot {};

    bot.listen_to_channel(channel_id, |msg| handle(&bot, msg))
        .await;
}

async fn handle<B: Bot>(bot: &B, msg: Message) {
    let client = bot.get_client();
    let channel_id = msg.channel_id;
    let content = msg.content;

    if msg.author.bot {
        log::debug!("message from bot");
        return;
    }

    let bot_name = std::env::var("BOT_NAME").unwrap_or(String::from("Chat assistant"));

    _ = client
        .edit_profile(
            serde_json::json!({ "username": bot_name })
                .as_object()
                .unwrap(),
        )
        .await;

    let co = ChatOptions {
        model: ChatModel::GPT35Turbo,
        restart: false,
        system_prompt: Some("You are a helpful assistant answering questions on Discord. If someone greets you without asking a question, you should simply respond \"Hello, I am your assistant on Discord, built by the Second State team. I am ready for your instructions now!\""),
    };

    let of = OpenAIFlows::new();

    if let Ok(c) = of
        .chat_completion(&channel_id.to_string(), &content, &co)
        .await
    {
        _ = client
            .send_message(
                channel_id.into(),
                &serde_json::json!({
                    "content": c.choice,
                }),
            )
            .await;
    }
}
