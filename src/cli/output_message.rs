use colored::*;

use crate::wgsl_error::WgslError;
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

    pub fn error(path: &Path, error: WgslError) -> Self {
        let err_text = match error {
            WgslError::ParserErr {
                error,
                line,
                pos,
            } => {
                let arrow = "-->".blue();
                let location = format!("{}:{}:{}", path.display(), line, pos);
                let error = format!("{}: {}", "error".red().bold(), error);

                format!("{} {}\n{}", arrow, location, error)
            }
            err => {
                format!("❌ {} \n{:#?}", path.display(), err)
            }
        };

        Self {
            is_err: true,
            text: err_text,
        }
    }
}
