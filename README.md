# <p align="center">A Discord ChatGPT bot</p>
<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/Discord-ChatGPT">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](https://flows.network/flow/createByTemplate/Discord-ChatGPT), and you will get a Discord bot that uses ChatGPT to respond to every question in your Discord DM.

## Prerequisites

* You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).
* You also need a bot token to access the Discord API. [How to create a Discord chat bot](https://flows.network/blog/discord-chat-bot-guide).

## Deploy your Telegram ChatGPT bot in 3 steps

1. Create a bot from a template
2. Add your ChatGPT API key
3. Add the Discord bot token

### 1 Create a bot from a template

[**Just click here**](https://flows.network/flow/createByTemplate/Discord-ChatGPT)

Here you can see three variables. You can customize the `system_prompt` variable to prompt ChatGPT. 

Click on the **Create and Build** button.

### 2 Add your OpenAI API key

You will now set up OpenAI integration. Click on **Connect**, and enter your key.

Close the tab and go back to the flow.network page once you are done. Click on **Continue**.

### 3 Add the Telegram bot token

You will now set up Discord integration. Enter your Discord token here.

Click on **Deploy** button.

## Give it a try. 

When the status of the flow is ready and running, you can invite the Discord bot to your sever.
Refer to this guide to [invite the bot to your server](https://flows.network/blog/discord-chat-bot-guide).

After the bot joined your server, you can find the bot on the right contact list and DM the bot. The bot will 
now answer your questions.

## Others

If you want to build locally, make sure you have installed Rust and added `wasm32-wasi` traget.

```
cargo build --target wasm32-wasi --release
```

