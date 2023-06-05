use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use reqwest::{self, header};
use std::io::{stdout, Write};
use std::fs;
use chatgpt::prelude::*;
use tokio::*;
use futures_util::StreamExt;

#[macro_use]
extern crate log;
extern crate env_logger;

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const CONTEXT_LOCATION: &str = "/Users/simonschaefer/dev/ai-projects/chat-bot/src/main.rs";

// Represents the  messages that the client and the LLM (Large Language Model) exchange.
#[derive(Serialize, Deserialize, Clone)]
struct Message {
    // who sent the message. The 'user' or the 'system', aka the LLM.
    role: String,
    content: String,
}

// API request
#[derive(Serialize, Clone)]
struct ChatRequest {
    // all messages the LLM should interpret.
    messages: Vec<Message>,
    model: String,
    max_tokens: i32,
    temperature: f64,
    top_p: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
}

// API response
#[derive(Deserialize, Clone)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Clone)]
struct Choice {
    message: Message,
}


#[tokio::main]
async fn main() -> Result<()> {

    env_logger::init();

    let api_key = read_api_key()?;
    debug!("api key: {}", api_key);

    let config = ModelConfiguration {
        engine: ChatGPTEngine::Gpt4,
        ..Default::default()
    };

    let client = ChatGPT::new_with_config(api_key, config)?;
    
    // the chatgpt api is stateless and does not have any context of previous messages. Therefore
    // the client needs to keep track of the state and add previous messages to each request.
    let mut chat_history: Vec<(Message, Message)> = vec![];
    let chat_context = load_context_from_file_and_return_as_messages();
    chat_history.push(chat_context);


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

        // add previously sent messages to the chat.
        let mut chat: Vec<Message>  = chat_history.iter()
            .flat_map(|(req, resp)| vec![req.clone(), resp.clone()])
            .collect();

        // add new user input to chat.
        chat.push(Message{role: "user".to_string(), content: input.trim().to_string()});

        let stream = client
            .send_message_streaming(input.trim().to_string())
            .await?;

        // debug!("{:?}", resp_body);

        println!(" --- ");
        println!("GPT-4-32k: \n --- ");

        stream.for_each(|each| async move {
            if let ResponseChunk::Content {
                delta,
                response_index: _,
            } = each
            {
                // Printing part of response without the newline
                print!("{delta}");
                // Manually flushing the standard output, as `print` macro does not do that
                stdout().lock().flush().unwrap();
            }
        })
        .await;

        println!();
    }

}

fn read_api_key() -> Result<String> {
    let api_key_file = std::env::var("HOME")? + "/dev/GPT-KEY";
    let api_key_bytes = std::fs::read_to_string(api_key_file)?;
    Ok(api_key_bytes.trim().to_string())
}

// the context is established by loading the file that is the context and creating a 'user' message
// from it. We also add a placeholder response from the system to it so we keep req/resp pairs.'
 fn load_context_from_file_and_return_as_messages() -> (Message, Message) {
    let context_text = match fs::read_to_string(CONTEXT_LOCATION) {
        Ok(text) => text,
        Err(e) => {
            error!("Error reading chat context from file: {}", e);
            String::new()
        }
    };

    let message_with_context = Message {
        role: "user".to_string(),
        content: format!("Remember the following code. Don't do anything with it until my next prompt yet.\n\n---\n\n{}", context_text)
    };

    let first_response = Message {
        role: "system".to_string(),
        content: "Alright, I won't do anything with the code yet. Just let me know what you would like me to do with it.".to_string()
    };

    (message_with_context, first_response)
}
