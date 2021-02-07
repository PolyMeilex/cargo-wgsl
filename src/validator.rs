use naga::front::wgsl;
use std::path::Path;

use crate::wgsl_error::WgslError;

pub struct Validator {
    validator: naga::proc::Validator,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            validator: naga::proc::Validator::new(),
        }
    }

    pub fn validate_wgsl(&mut self, path: &Path) -> Result<(), WgslError> {
        let shader = std::fs::read_to_string(&path).map_err(WgslError::from)?;
        let module = wgsl::parse_str(&shader).map_err(WgslError::from)?;

        if let Err(err) = self.validator.validate(&module) {
            Err(WgslError::ValidationErr(err))
        } else {
            Ok(())
        }
    }
}
