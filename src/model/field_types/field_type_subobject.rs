use crate::model::field_types::{
    check_if_required, check_reference_exists, check_reference_is_authorized, run_validators,
    ReturnHitError,
};

use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::{ObjectValue, Reference};
use crate::HitError;

pub struct FieldTypeSubobject {
    pub required: bool,
    pub name: String,
    pub validators: Validators<Reference>,
    pub authorized_models: Vec<String>,
}

impl ModelField for FieldTypeSubobject {
    fn accepts_for_set(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            // ObjectValue::Null => !self.required,
            // ObjectValue::SubObject(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, model: &Model) -> bool {
        match check_reference_is_authorized(&self.authorized_models, model) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    fn validate(&self, value: &ObjectValue, context: &ValidatorContext) -> ReturnHitError {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::Reference(value) => {
                let mut errors: Vec<HitError> = vec![];
                //verify validity of reference
                let entry = check_reference_exists(value, context)?;
                check_reference_is_authorized(&self.authorized_models, &entry.get_model())?;
                //Run validators
                run_validators(&self.validators, value, &mut errors, context);

                if errors.len() > 0 {
                    return Err(errors);
                }
                return Ok(());
            }
            _ => Err(vec![HitError::InvalidDataType()]),
        }
    }
    fn is_vec_reference(&self) -> bool {
        false
    }
    fn is_vec_subobject(&self) -> bool {
        false
    }
}
