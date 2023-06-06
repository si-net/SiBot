use std::io::{self, BufRead};
use std::io::{stdout, Write};
use std::fs;
use chatgpt::prelude::*;
use chatgpt::types::*;
use futures_util::StreamExt;
use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
extern crate log;
extern crate env_logger;

#[derive(Debug, StructOpt)]
#[structopt(name = "chatgpt-client", about = "A command line client for ChatGPT")]
struct Opt {
    #[structopt(long, parse(from_os_str), help = "Path to the file that should be the topic of the conversation", default_value = "/Users/simonschaefer/dev/ai-projects/chat-bot/src/main.rs")]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {

    env_logger::init();

    let opt = Opt::from_args();

    let api_key = read_api_key()?;
    debug!("api key: {}", api_key);

    let config = ModelConfiguration {
        engine: ChatGPTEngine::Gpt4,
        ..Default::default()
    };

    let client = ChatGPT::new_with_config(api_key, config)?;

    // the chatgpt api is stateless and does not have any context of previous messages. Therefore
    // the client needs to keep track of the state and add previous messages to each request.
    let mut conversation: Conversation = client.new_conversation();
    
    // Add the file context to the conversation
    let history = load_context_from_file_and_return_as_messages(opt.file);
    conversation.history.push(history.0);
    conversation.history.push(history.1);

    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    // main program loop: exchanges messages between user and LLM.
    loop {
        // wait for user input.
        println!("You: ");
        io::stdout().flush()?;
        let input = reader.next().unwrap()?;
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut stream = conversation
            .send_message_streaming(input.trim().to_string())
            .await?;

        // debug!("{:?}", resp_body);

        println!(" --- ");
        println!("GPT-4-8k: \n");

        // We want to stream the response from chatgpt to the console. This means that we have to
        // jump through some hoops, to store the complete response and save it to the
        // conversation.history.
        let mut output: Vec<ResponseChunk> = Vec::new();
        
        while let Some(chunk) = stream.next().await {
            match chunk {
                ResponseChunk::Content {
                    delta,
                    response_index,
                } => {
                    // Printing part of response without the newline
                    print!("{delta}");
                    // Manually flushing the standard output, as `print` macro does not do that
                    stdout().lock().flush().unwrap();
                    
                    output.push(ResponseChunk::Content {
                        delta,
                        response_index,
                    });
                }
                // We don't really care about other types, other than parsing them into a ChatMessage later
                other => output.push(other),
            }
        }
        println!("\n---");
       // Parsing ChatMessage from the response chunks and saving it to the conversation history
        let messages = ChatMessage::from_response_chunks(output);
        conversation.history.push(messages[0].to_owned());    
    }
}

fn read_api_key() -> Result<String> {
    let api_key_file = std::env::var("HOME")? + "/dev/GPT-KEY";
    let api_key_bytes = std::fs::read_to_string(api_key_file)?;
    Ok(api_key_bytes.trim().to_string())
}

// the context is established by loading the file that is the context and creating a 'user' message
// from it. We also add a placeholder response from the system to it so we keep req/resp pairs.'
 fn load_context_from_file_and_return_as_messages(file_path: PathBuf) -> (ChatMessage, ChatMessage) {
    let context_text = match fs::read_to_string(&file_path) {
        Ok(text) => text,
        Err(e) => {
            error!("Error reading chat context from file: {}, {}", file_path.display(), e);
            String::new()
        }
    };

    let message_with_context = ChatMessage {
        role: Role::User,
        content: format!("Remember the following code. Don't do anything with it until my next prompt yet.\n\n---\n\n{}", context_text)
    };

    let first_response = ChatMessage {
        role: Role::System,
        content: "Alright, I won't do anything with the code yet. Just let me know what you would like me to do with it.".to_string()
    };

    (message_with_context, first_response)
}
