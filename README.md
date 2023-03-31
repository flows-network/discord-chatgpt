# <p align="center">A ChatGPT Bot for your Discord server</p>



<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>



[Deploy this function on flows.network](#deploy-the-chatgpt-discord-bot), and you will get a Discord bot that uses ChatGPT to respond to every question in your Discord server automatically.

<img width="881" alt="image" src="https://user-images.githubusercontent.com/45785633/229087866-dac567de-c060-4cad-9047-01819e34c806.png">
(See an example)



## Prerequisite 

You will need an [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

## Deploy the ChatGPT Discord bot

To install the ChatGPT Slack App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/discord-chatgpt) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Get Started" button and go to a new page, "My flows".
3. Click on the "Create a Flow" button to start deploying the ChatGPT Discord bot.
4. Authenticate the [flows.network](https://flows.network/) to access the `discord-chatgpt` repo you just forked. Don't forget to choose "With Environment Variables", which we will configure the required parameters. Once done, click on the "Next" button.
<img width="779" alt="image" src="https://user-images.githubusercontent.com/45785633/224656121-98d146ae-8bab-40b1-a589-057a34572884.png">

5. Fill in the required Environment Variables. In this example, we have three variables. One is `server_name`. Fill in the Discord server Name you want to connect here. The second one is `channel`. Fill in the Slack channel under the Discord server you just entered. The last one is `openai_key_name`. Fill in the name you want to name your OpenAI Key.

![image](https://user-images.githubusercontent.com/45785633/229090360-b38f4ec4-6aee-4dfa-8056-dd1ead5800e3.png)


6. Name your flow, and click on "Turn on and Save" button to deploy the flow function.


### Configure SaaS integrations

After that, the flows.network will redirect you to the flow details page automatically. In the Flow details tab, we can set up SaaS integrations required by the flow.

<img width="955" alt="image" src="https://user-images.githubusercontent.com/45785633/229090684-e0005042-f826-4c18-baa6-2999bec906ba.png">


1. Click on the "Connect" button to authenticate your OpenAI account. You'll be redirected to a new page where you could copy and paste your OpenAI API key and then name the key. Note that the name you enter here should be the same as the name in the environment variables.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click the "Connect" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a Discord server. This server is the one you entered into the environment variables above.

That's all. As soon as the flow function's status becomes `ready`, the ChatGPT Discord App goes live. Go ahead and chat with ChatGPT by sending a message in the channel!

![image](https://user-images.githubusercontent.com/45785633/229091045-e0798b17-fac7-40bd-8f33-8a0f1621c265.png)


> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!

## Others

If you want to build locally, make sure you have installed Rust and added `wasm32-wasi` traget.

```
cargo build --target wasm32-wasi --release
```

