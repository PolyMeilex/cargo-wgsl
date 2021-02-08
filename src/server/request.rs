use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub event: RequestEvent,
}

#[derive(Serialize, Deserialize)]
pub enum RequestEvent {
    Error,
    ValidatePath(PathBuf),
}
