use std::path::PathBuf;

use crate::naga::Naga;

use jsonrpc_stdio_server::jsonrpc_core::*;
use jsonrpc_stdio_server::ServerBuilder;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ValidateFileParams {
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
enum ValidateFileResponse {
    Ok(bool),
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
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut io = IoHandler::default();

        // Protocol Version
        io.add_sync_method("version", move |_| Ok(Value::from("0.0.1")));
        // Binary Version
        io.add_sync_method("binary_version", move |_| Ok(Value::from("0.0.4")));

        io.add_sync_method("get_file_tree", move |params: Params| {
            let params: ValidateFileParams = params.parse()?;

            let mut naga = Naga::new();

            let tree = naga.get_wgsl_tree(&params.path).ok();

            Ok(to_value(tree).unwrap())
        });

        io.add_sync_method("validate_file", move |params: Params| {
            let params: ValidateFileParams = params.parse()?;

            let mut validator = Naga::new();

            let res = match validator.validate_wgsl(&params.path) {
                Ok(_) => ValidateFileResponse::Ok(true),
                Err(err) => {
                    use crate::wgsl_error::WgslError;
                    match err {
                        WgslError::ParserErr { error, line, pos } => {
                            ValidateFileResponse::ParserErr {
                                error,
                                scopes: vec![],
                                line,
                                pos,
                            }
                        }
                        WgslError::ValidationErr { src, error, .. } => {
                            if let Some((span, _)) = error.spans().next() {
                                let loc = span.location(&src);
                                ValidateFileResponse::ParserErr {
                                    error: format!("{}.\n\n{:#?}", error, error),
                                    scopes: vec![],
                                    line: loc.line_number as usize,
                                    pos: loc.line_position as usize,
                                }
                            } else {
                                ValidateFileResponse::ValidationErr {
                                    message: format!("{}.\n\n{:#?}", error, error),
                                    debug: format!("{:#?}", error),
                                }
                            }
                        }
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
