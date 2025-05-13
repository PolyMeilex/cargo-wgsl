use naga::{front::wgsl::ParseError, valid::ValidationError, WithSpan};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum WgslError {
    ValidationErr {
        src: String,
        error: WithSpan<ValidationError>,
        emitted: String,
    },
    ParserErr {
        error: String,
        line: usize,
        pos: usize,
    },
    #[allow(unused)]
    IoErr(std::io::Error),
}

impl From<std::io::Error> for WgslError {
    fn from(err: std::io::Error) -> Self {
        Self::IoErr(err)
    }
}

impl WgslError {
    pub fn from_parse_err(err: ParseError, src: &str) -> Self {
        let error = err.emit_to_string(src);
        let loc = err.location(src);
        if let Some(loc) = loc {
            Self::ParserErr {
                error,
                line: loc.line_number as usize,
                pos: loc.line_position as usize,
            }
        } else {
            Self::ParserErr {
                error,
                line: 0,
                pos: 0,
            }
        }
    }
}
