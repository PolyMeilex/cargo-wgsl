mod output_message;
use output_message::OutputMessage;

use crate::naga::Naga;
use walkdir::WalkDir;

use std::io::Read;
use std::path::Path;
use std::string::String;

pub fn run() -> i32 {
    let root_dir = std::fs::canonicalize("./").unwrap();

    let mut validator = Naga::new();

    let mut messages = Vec::new();

    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--stdin".to_string()) {
        let mut buffer = String::new();
        let size = std::io::stdin().read_to_string(&mut buffer).unwrap_or(0);
        if size > 0 {
            let msg = match validator.validate_wgsl(buffer.as_str()) {
                Ok(_) => OutputMessage::success(Path::new("@stdin")),
                Err(err) => OutputMessage::error(Path::new("@stdin"), err),
            };
            messages.push(msg);
        }
    } else {
        let dir_walk = WalkDir::new(&root_dir);
        let dir_walk = dir_walk.into_iter().filter_entry(|e| {
            let path = e.path();

            if !path.is_dir() {
                path.extension().map(|ext| &*ext == "wgsl").unwrap_or(false)
            } else {
                true
            }
        });

        for entry in dir_walk {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if !path.is_dir() {
                        let msg = match validator.validate_wgsl_file(path) {
                            Ok(_) => {
                                let path = path.strip_prefix(&root_dir).unwrap_or(path);
                                OutputMessage::success(path)
                            }
                            Err(err) => {
                                let path = path.strip_prefix(&root_dir).unwrap_or(path);
                                OutputMessage::error(path, err)
                            }
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
