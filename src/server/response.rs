use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub event: ResponseEvent,
}

#[derive(Serialize, Deserialize)]
pub enum ResponseEvent {
    Ok,
    ParserErr {
        error: String,
        scopes: Vec<String>,
        line: usize,
        pos: usize,
    },
    UnknownError(String),
}
