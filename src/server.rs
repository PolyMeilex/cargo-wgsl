use std::path::PathBuf;

use crate::validator::Validator;

use jsonrpc_stdio_server::jsonrpc_core::*;
use jsonrpc_stdio_server::ServerBuilder;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ValidateFileParams {
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
enum ValidateFileResponse {
    Ok,
    ParserErr {
        error: String,
        scopes: Vec<String>,
        line: usize,
        pos: usize,
    },
    ValidationErr {
        message: String,
        debug: String,
    },
    UnknownError(String),
}

pub fn run() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut io = IoHandler::default();
        io.add_sync_method("validate_file", move |params: Params| {
            let params: ValidateFileParams = params.parse()?;

            let mut validator = Validator::new();

            let res = match validator.validate_wgsl(&params.path) {
                Ok(_) => ValidateFileResponse::Ok,
                Err(err) => {
                    use crate::wgsl_error::WgslError;
                    match err {
                        WgslError::ParserErr {
                            error,
                            scopes,
                            line,
                            pos,
                        } => ValidateFileResponse::ParserErr {
                            error,
                            scopes: scopes.into_iter().map(|s| format!("{:?}", s)).collect(),
                            line,
                            pos,
                        },
                        WgslError::ValidationErr(err) => ValidateFileResponse::ValidationErr {
                            message: format!("{}", err),
                            debug: format!("{:#?}", err),
                        },
                        err => ValidateFileResponse::UnknownError(format!("{:#?}", err)),
                    }
                }
            };

            Ok(to_value(res).unwrap())
        });

        let server = ServerBuilder::new(io).build();
        server.await;
    })
}
