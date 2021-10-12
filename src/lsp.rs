use std::{error::Error, fs::File, path::Path};

use lsp_types::{
    notification::{
        DidOpenTextDocument, DidSaveTextDocument, Notification as _, PublishDiagnostics,
    },
    request::GotoDefinition,
    DiagnosticSeverity, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    GotoDefinitionResponse, InitializeParams, SaveOptions, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
};

use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use naga::front::wgsl;
use simplelog::*;

use crate::naga::{NagaValidator, WgslSource};

pub fn run() -> Result<(), Box<dyn Error + Sync + Send>> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("/home/poly/Documents/Programing/rust/cargo-wgsl/log.log").unwrap(),
    )])
    .unwrap();

    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::Incremental),
                will_save: None,
                will_save_wait_until: None,
                save: Some(SaveOptions::default().into()),
            },
        )),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    log::info!("starting example main loop");

    let mut naga = NagaValidator::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                log::info!("got request: {:?}", req);
                match cast::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        log::info!("got gotoDefinition request #{}: {:?}", id, params);
                        let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(req) => req,
                };
            }
            Message::Response(resp) => {
                log::info!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                log::info!("got notification: {:?}", not);

                match not.method.as_str() {
                    DidOpenTextDocument::METHOD => {
                        if let Ok(params) =
                            not.extract::<DidOpenTextDocumentParams>(DidOpenTextDocument::METHOD)
                        {
                            let res =
                                validate_file(&mut naga.validator, params.text_document.uri.path());

                            let diagnostics = if let Err(diagnostic) = res {
                                vec![diagnostic]
                            } else {
                                vec![]
                            };

                            let params = lsp_types::PublishDiagnosticsParams {
                                uri: params.text_document.uri,
                                diagnostics,
                                version: None,
                            };

                            let resp = new_notification::<PublishDiagnostics>(params);

                            connection.sender.send(Message::Notification(resp))?;
                        };
                    }
                    DidSaveTextDocument::METHOD => {
                        if let Ok(params) =
                            not.extract::<DidSaveTextDocumentParams>(DidSaveTextDocument::METHOD)
                        {
                            let res =
                                validate_file(&mut naga.validator, params.text_document.uri.path());

                            let diagnostics = if let Err(diagnostic) = res {
                                vec![diagnostic]
                            } else {
                                vec![]
                            };

                            let params = lsp_types::PublishDiagnosticsParams {
                                uri: params.text_document.uri,
                                diagnostics,
                                version: None,
                            };

                            let resp = new_notification::<PublishDiagnostics>(params);

                            connection.sender.send(Message::Notification(resp))?;
                        };
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), Request>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn new_notification<N: lsp_types::notification::Notification>(params: N::Params) -> Notification {
    let params = serde_json::to_value(params).unwrap();
    Notification {
        method: N::METHOD.to_owned(),
        params,
    }
}

fn validate_file<P: AsRef<Path>>(
    validator: &mut naga::valid::Validator,
    path: P,
) -> Result<(), lsp_types::Diagnostic> {
    let path = path.as_ref();
    match WgslSource::from(&path) {
        Ok(source) => match wgsl::parse_str(&source.code) {
            Ok(module) => {
                if let Err(error) = validator.validate(&module) {
                    let diag = lsp_types::Diagnostic {
                        message: format!("{:#?}", error),
                        severity: Some(DiagnosticSeverity::Error),
                        source: Some("wgsl".to_string()),
                        ..Default::default()
                    };
                    return Err(diag);
                }
            }
            Err(error) => {
                let (line, pos) = error.location(&source.code);

                let diag = lsp_types::Diagnostic {
                    message: error.emit_to_string(&source.code),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wgsl".to_string()),
                    range: lsp_types::Range {
                        start: lsp_types::Position {
                            line: line as u32 - 1,
                            character: pos as u32,
                        },
                        end: lsp_types::Position {
                            line: line as u32 - 1,
                            character: pos as u32,
                        },
                    },
                    ..Default::default()
                };
                return Err(diag);
            }
        },
        Err(error) => {
            let diag = lsp_types::Diagnostic {
                message: format!("{:#?}", error),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wgsl".to_string()),
                ..Default::default()
            };
            return Err(diag);
        }
    };

    Ok(())
}
