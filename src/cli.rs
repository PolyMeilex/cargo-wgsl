mod output_message;
use naga::front::wgsl;
use output_message::OutputMessage;

use crate::naga::{NagaValidator, WgslSource};
use walkdir::WalkDir;

pub fn run() -> i32 {
    let root_dir = std::fs::canonicalize("./").unwrap();

    let mut validator = NagaValidator::new();

    let dir_walk = WalkDir::new(&root_dir);
    let dir_walk = dir_walk.into_iter().filter_entry(|e| {
        let path = e.path();

        if !path.is_dir() {
            path.extension().map(|ext| &*ext == "wgsl").unwrap_or(false)
        } else {
            true
        }
    });

    let mut messages = Vec::new();

    for entry in dir_walk {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_dir() {
                    let msg = match WgslSource::from(&path) {
                        Ok(source) => match wgsl::parse_str(&source.code) {
                            Ok(module) => {
                                if let Err(error) = validator.validator.validate(&module) {
                                    OutputMessage::unknown_error(path, error)
                                } else {
                                    OutputMessage::success(path)
                                }
                            }
                            Err(error) => OutputMessage::parser_error(&source, error),
                        },
                        Err(error) => OutputMessage::unknown_error(path, error),
                    };

                    messages.push(msg);
                }
            }
            Err(err) => {
                messages.push(OutputMessage {
                    is_err: true,
                    text: format!("{:?}", err),
                });
            }
        }
    }

    messages.sort_by(|a, b| {
        if a.is_err && b.is_err {
            std::cmp::Ordering::Equal
        } else if a.is_err {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    let mut exit_code = 0;

    for msg in messages {
        println!("{}", msg.text);
        if msg.is_err {
            exit_code = 1;
        }
    }

    exit_code
}
