use naga::{
    front::wgsl::{ParseError, Scope},
    proc::ValidationError,
};

#[derive(Debug)]
pub enum WgslError {
    ValidationErr(ValidationError),
    ParserErr {
        error: String,
        scopes: Vec<Scope>,
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
        let ParseError {
            error,
            scopes,
            line,
            pos,
        } = err;
        let error = error.to_owned();

        Self::ParserErr {
            error: error.to_string(),
            scopes,
            line,
            pos,
        }
    }
}
