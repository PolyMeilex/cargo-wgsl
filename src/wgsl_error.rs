use naga::{
    front::wgsl::ParseError,
    valid::ValidationError,
};

#[derive(Debug)]
pub enum WgslError {
    ValidationErr(ValidationError),
    ParserErr {
        error: String,
        line: usize,
        pos: usize,
    },
    IoErr(std::io::Error),
}

impl From<std::io::Error> for WgslError {
    fn from(err: std::io::Error) -> Self {
        Self::IoErr(err)
    }
}

impl<'a> From<ParseError<'a>> for WgslError {
    fn from(err: ParseError<'a>) -> Self {
        let (line, pos) = err.location();
        let error = err.emit_to_string();
        Self::ParserErr {
            error,
            line,
            pos,
        }
    }
}
