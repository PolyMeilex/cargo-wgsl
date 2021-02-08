use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use std::io::prelude::*;

use crate::validator::Validator;

#[derive(Serialize, Deserialize)]
struct Request {
    event: RequestEvent,
}

#[derive(Serialize, Deserialize)]
enum RequestEvent {
    Error,
    ValidatePath(PathBuf),
}

#[derive(Serialize, Deserialize)]
struct Response {
    event: ResponseEvent,
}

#[derive(Serialize, Deserialize)]
enum ResponseEvent {
    Ok,
    ParserErr {
        error: String,
        scopes: Vec<String>,
        line: usize,
        pos: usize,
    },
    UnknownError(String),
}

impl Response {
    fn new(event: ResponseEvent) -> Self {
        Self { event }
    }
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

fn handle_input(input: &str) -> Result<Request, Response> {
    serde_json::from_str(&input).map_err(|err| {
        Response::new(ResponseEvent::UnknownError(format!(
            "Input parsing error: {:#?}",
            err
        )))
    })
}

pub fn run() {
    let stdin = std::io::stdin();
    let mut validator = Validator::new();

    let mut input = String::new();
    loop {
        if stdin.lock().read_line(&mut input).unwrap_or(0) > 0 {
            let res = match handle_input(&input) {
                Ok(request) => match request.event {
                    RequestEvent::ValidatePath(path) => match validator.validate_wgsl(&path) {
                        Ok(_) => Response::new(ResponseEvent::Ok),
                        Err(err) => {
                            use crate::wgsl_error::WgslError;
                            match err {
                                WgslError::ParserErr {
                                    error,
                                    scopes,
                                    line,
                                    pos,
                                } => Response::new(ResponseEvent::ParserErr {
                                    error,
                                    scopes: scopes
                                        .into_iter()
                                        .map(|s| format!("{:?}", s))
                                        .collect(),
                                    line,
                                    pos,
                                }),
                                err => {
                                    Response::new(ResponseEvent::UnknownError(format!("{:?}", err)))
                                }
                            }
                        }
                    },
                    _ => Response::new(ResponseEvent::UnknownError("Request Parse Failed".into())),
                },
                Err(response) => response,
            };

            println!("{}", res.to_json());

            input.clear();
        }
    }
}
