use crate::{
    errors::ValidationError,
    model::field_types::{check_if_required, run_validators},
    HitError,
};

use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::ObjectValue;
use std::default::Default;

#[derive(Default)]
pub struct FieldTypeStringVec {
    pub required: bool,
    pub name: String,
    pub validators: Validators<String>,
    pub _enum: Option<Vec<String>>,
}

impl ModelField for FieldTypeStringVec {
    fn on_kernel_init(&mut self, model_name: &str) -> Result<(), HitError> {
        for validator in self.validators.iter_mut() {
            validator.on_kernel_init(&self.name, model_name)?;
        }
        Ok(())
    }
    fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    fn accepts_for_set(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            ObjectValue::String(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, _model: &Model) -> bool {
        return false;
    }

    fn validate(
        &self,
        value: &ObjectValue,
        context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::String(value) => {
                let mut errors: Vec<ValidationError> = vec![];
                run_validators(&self.validators, value, &mut errors, context)?;

                if errors.len() > 0 {
                    return Ok(Some(errors));
                }
                return Ok(None);
            }
            _ => Err(HitError::InvalidDataType()),
        }
    }
    fn is_vec_reference(&self) -> bool {
        false
    }
    fn is_vec_subobject(&self) -> bool {
        false
    }
}
