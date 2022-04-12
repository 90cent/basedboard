use std::io::Error;

use rocket_contrib::json::Json;
use serde::Deserialize;
use log;

#[derive(Deserialize)]
pub struct ChatMessage {
    content: String
}

#[post("/chat", format="json", data = "<message>")]
pub fn chat(message: Json<ChatMessage>) -> Result<String,Error> {
    log::warn!("NEW POST REQUEST\nContent:{}",message.content);

    

    Ok(String::from("ok bruh"))
}