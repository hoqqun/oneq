use chatgpt::{prelude::*, types::CompletionResponse};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Select gpt engine. default is gpt35turbo, options are gpt35turbo, gpt4, gpt4o"
    )]
    model: Option<String>,

    #[arg(short, long, help = "Open AI API key")]
    apikey: String,

    #[arg(short, long, help = "Question statement that you want to ask")]
    question: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let engine = if let Some(value) = args.model {
        match &value[..] {
            "gpt35turbo" => ChatGPTEngine::Gpt35Turbo,
            "gpt4" => ChatGPTEngine::Gpt4,
            "gpt4o" => ChatGPTEngine::Custom("gpt-4o"),
            _ => panic!("Select option from gpt35turbo or gpt4 or gpt4o !"),
        }
    } else {
        ChatGPTEngine::Gpt35Turbo
    };

    let api_key = args.apikey.clone();
    let question = args.question.clone();

    match chatgpt(&api_key, &engine, &question).await {
        Ok(content) => println!("ChatGPT: {}", content),
        Err(value) => println!("{:?}", value),
    }
}

async fn chatgpt(api_key: &String, engine: &ChatGPTEngine, question: &String) -> Result<String> {
    // let gpt4o = ChatGPTEngine::Custom("gpt-4o");
    let client = ChatGPT::new_with_config(
        api_key,
        ModelConfigurationBuilder::default()
            .temperature(0.7)
            .engine(*engine)
            .build()
            .unwrap(),
    )?;

    let response: CompletionResponse = client.send_message(question).await?;

    Ok(response.message().content.clone())
}
