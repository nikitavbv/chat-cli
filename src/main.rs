use {
    clap::Parser,
    async_openai::{
        Client,
        config::OpenAIConfig,
        types::{
            CreateChatCompletionRequest,
            ChatCompletionRequestMessage,
            ChatCompletionRequestUserMessage,
            ChatCompletionRequestUserMessageContent,
            Role,
        },
    },
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

    println!("running");
    let res = client.chat().create(CreateChatCompletionRequest {
        messages: vec![
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Text("Hello!".to_owned()),
                role: Role::User,
                ..Default::default()
            }),
        ],
        ..Default::default()
    }).await.unwrap();

    println!("result: {:?}", res);

    Ok(())
}
