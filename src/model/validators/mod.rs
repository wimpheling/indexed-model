use crate::{errors::ValidationError, Hit};
use crate::{errors::ValidationErrorLevel, HitError};
use std::rc::Rc;

pub type Validators<T> = Vec<Box<dyn Validator<T>>>;

pub trait Validator<T> {
    fn validate(
        &self,
        value: &T,
        context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError>;

    fn on_kernel_init(&mut self, field_name: &str, model_name: &str) -> Result<(), HitError>;
}

pub struct MaxLength {
    pub length: u8,
}
pub static ERROR_MAX_LENGTH: &str = "Max length was reached.";

pub struct ValidatorContext<'a> {
    pub id: &'a str,
    pub property: &'a str,
    pub index: Rc<&'a Hit>,
}

static MAX_LENGTH: &str = "MAX_LENGTH";

impl Validator<String> for MaxLength {
    fn validate(
        &self,
        value: &String,
        _context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        if value.len() as u8 > self.length {
            return Ok(Some(vec![ValidationError {
                key: MAX_LENGTH.to_string(),
                level: ValidationErrorLevel::Error,
                arguments: None,
            }]));
        }
        return Ok(None);
    }

    fn on_kernel_init(&mut self, _field_name: &str, _model_namee: &str) -> Result<(), HitError> {
        Ok(())
    }
}
