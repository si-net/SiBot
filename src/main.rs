use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use reqwest::{self, header};
use std::io::Write;
use std::fs;

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const CONTEXT_LOCATION: &str = "src/main.rs";

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Clone)]
struct ChatRequest {
    messages: Vec<Message>,
    model: String,
    max_tokens: i32,
    temperature: f64,
    top_p: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
}

#[derive(Deserialize, Clone)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Clone)]
struct Choice {
    message: Message,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = read_api_key()?;

    let mut chat_history: Vec<(Message, Message)> = vec![];

    let chat_context = load_context_from_file_and_return_as_messages();
    chat_history.push(chat_context);


    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    loop {
        print!("You: ");
        io::stdout().flush()?;
        let input = reader.next().unwrap()?;
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut messages: Vec<Message>  = chat_history.iter()
            .flat_map(|(req, resp)| vec![req.clone(), resp.clone()])
            .collect();

        messages.push(Message{role: "user".to_string(), content: input.trim().to_string()});

        let chat_req = ChatRequest {
            messages: messages,
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 300,
            temperature: 0.7,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        };
        let req_body = serde_json::to_string(&chat_req)?;

        let client = reqwest::blocking::Client::new();
        let resp = client.post(ENDPOINT)
            .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .body(req_body.clone())
            .send()?;
        let resp_body: Value = resp.json()?;
        
        let chat_resp: ChatResponse = serde_json::from_value(resp_body.clone())?;

        if let Some(choice) = chat_resp.choices.first() {
            chat_history.push( (chat_req.messages.first().unwrap().clone(), choice.message.clone()) );
            println!(" --- ");
            println!("GPT-3.5: {}\n --- ", choice.message.content);
        } else {
            println!("Error: {:?}", resp_body);
        }
    }

}

fn read_api_key() -> Result<String, Box<dyn std::error::Error>> {
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
            println!("Error reading chat context from file: {}", e);
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
