use {
    async_openai::{
        config::OpenAIConfig, types::{
            ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest, Role
        }, Client
    }, clap::Parser, std::io::{self, BufRead, Write}
};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    endpoint: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let config = OpenAIConfig::new()
        .with_api_base(args.endpoint);
    let client = Client::with_config(config);

    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let mut messages = Vec::new();

    loop {
        stdout.write("You: ".as_bytes()).unwrap();
        stdout.flush().unwrap();

        let input = stdin.lock().lines().next().unwrap().unwrap();
        messages.push(ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(input),
            role: Role::User,
            ..Default::default()
        }));

        let response = client.chat().create(CreateChatCompletionRequest {
            messages: messages.clone(),
            ..Default::default()
        }).await.unwrap().choices.get(0).unwrap().message.content.as_ref().unwrap().clone();

        println!("Assistant: {}", response);

        messages.push(ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage {
            content: Some(response),
            role: Role::Assistant,
            ..Default::default()
        }));
    }
}
