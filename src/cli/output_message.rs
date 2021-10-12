use colored::*;
use naga::front::wgsl::ParseError;

use crate::naga::WgslSource;
use std::path::Path;

pub struct OutputMessage {
    pub is_err: bool,
    pub text: String,
}

impl OutputMessage {
    pub fn success(path: &Path) -> Self {
        let succes = "Success".bright_green().bold();
        OutputMessage {
            is_err: false,
            text: format!("✅ {} {}", succes, path.display()),
        }
    }

    pub fn parser_error(source: &WgslSource, error: ParseError) -> Self {
        let (line, pos) = error.location(&source.code);
        let error = error.emit_to_string(&source.code);

        let err_text = {
            let arrow = "-->".blue();
            let location = format!("{}:{}:{}", source.path.display(), line, pos);
            let error = format!("{}: {}", "error".red().bold(), error);

            format!("{} {}\n{}", arrow, location, error)
        };

        Self {
            is_err: true,
            text: err_text,
        }
    }

    pub fn unknown_error<D: std::fmt::Debug>(path: &Path, error: D) -> Self {
        let err_text = format!("❌ {} \n{:#?}", path.display(), error);

        Self {
            is_err: true,
            text: err_text,
        }
    }
}
