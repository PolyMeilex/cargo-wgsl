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
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut io = IoHandler::default();

        io.add_sync_method("version", move |_| Ok(Value::from("0.0.1")));

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
                        WgslError::ParserErr {
                            error,
                            line,
                            pos,
                        } => ValidateFileResponse::ParserErr {
                            error,
                            scopes: vec![],
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
