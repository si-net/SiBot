use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use reqwest::{self, header};
use std::io::Write;

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Clone)]
struct ChatRequest<'a> {
    messages: Vec<Message>,
    model: &'a str,
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

    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    let mut chat_history: Vec<(ChatRequest, ChatResponse)> = vec![];

    loop {
        print!("You: ");
        io::stdout().flush()?;
        let input = reader.next().unwrap()?;
        let input = input.trim();
        if input.is_empty() {
            continue;
        }


        let messages = vec![Message {
            role: "user".to_string(),
            content: input.trim().to_string(),
        }];

        let chat_req = ChatRequest {
            messages: messages.clone(),
            model: "gpt-3.5-turbo",
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

        chat_history.push((chat_req.clone(), chat_resp.clone()));

        if let Some(choice) = chat_resp.choices.first() {
            // TODO: make this into debug logging. Idea: make file in debug mode.
            println!("GPT-3.5: {}", choice.message.content);
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

