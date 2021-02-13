mod output_message;
use output_message::OutputMessage;

use crate::naga::Naga;
use walkdir::WalkDir;

pub fn run() {
    let root_dir = std::fs::canonicalize("./").unwrap();

    let mut validator = Naga::new();

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
                    let msg = match validator.validate_wgsl(&path) {
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

    messages.sort_by(|a, b| {
        if a.is_err && b.is_err {
            std::cmp::Ordering::Equal
        } else if a.is_err {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    let messages: Vec<String> = messages.into_iter().map(|msg| msg.text).collect();

    for msg in messages {
        println!("{}", msg);
    }
}
