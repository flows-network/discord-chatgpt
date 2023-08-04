# <p align="center">A Discord ChatGPT bot</p>
<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/discord-chatgpt">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](https://flows.network/flow/createByTemplate/discord-chatgpt), and you will get a Discord bot that uses ChatGPT to respond to every question in your Discord DM.

## Prerequisites

* You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).
* You also need a bot token to access the Discord API. Please refer to [How to create a Discord chat bot](https://flows.network/blog/discord-chat-bot-guide) to save the Discord API Token and invite the bot to your server. You can see the bot is offline now. But don't worry! After you finish all the steps, the bot will be online. 

## Deploy your Telegram ChatGPT bot in 3 steps

1. Create a bot from a template
2. Add your ChatGPT API key
3. Add the Discord bot token

### 1 Create a bot from a template

[**Just click here**](https://flows.network/flow/createByTemplate/discord-chatgpt)

<img width="500" alt="image" src="https://github.com/flows-network/discord-chatgpt/assets/45785633/f53553da-ccec-4fa3-ada3-851d7c3b4c5e">

Here you can see three variables. You can customize the `system_prompt` variable to prompt ChatGPT. 

Click on the **Create and Build** button.

### 2 Add the Discord token

You will now set up Discord integration. Enter your Discord token here. Please refer to [How to create a Discord chat bot](https://flows.network/blog/discord-chat-bot-guide).

<img width="530" alt="image" src="https://github.com/flows-network/discord-chatgpt/assets/45785633/e978d2fe-5666-4862-84c6-91a87a25e929">

Click on **Continue**.

### 3 Add your OpenAI API key

You will now set up OpenAI integration. Click on **Connect**, and enter your key.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/226564674-902933b5-8ff3-4724-93e3-2b2f67dc0b9a.png">](https://user-images.githubusercontent.com/45785633/226564674-902933b5-8ff3-4724-93e3-2b2f67dc0b9a.png)

Close the tab and go back to the flow.network page once you are done. Click on **Deploy** button.

## Give it a try. 

When the status of the flow is ready and running, you can go back to your Discord server and check out the status of the bot you just invited.

Now you can see the bot on the right `online` contact list. DM the bot or @the bot in a channel, the bot will answer your questions.

## Others

If you want to build locally, make sure you have installed Rust and added `wasm32-wasi` traget.

```
cargo build --target wasm32-wasi --release
```

