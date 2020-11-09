use std::{cell::RefCell, clone::Clone, rc::Rc};

use crate::{
    model::validators::{Validator, ValidatorContext},
    ObjectValue, Reference,
};
use crate::{HitError, ValidationError};

use super::unique_in_parent_plugin::{UniqueInParentPlugin, UniqueInParentValueIndexValue};
static UNIQUE_IN_PARENT: &str = "UNIQUE_IN_PARENT";

pub struct UniqueInParentValidator {
    property_name: String,
    index: Rc<RefCell<UniqueInParentPlugin>>,
}

impl UniqueInParentValidator {
    pub fn new(
        property_name: String,
        index: Rc<RefCell<UniqueInParentPlugin>>,
    ) -> Rc<RefCell<UniqueInParentValidator>> {
        Rc::new(RefCell::new(UniqueInParentValidator {
            property_name: property_name,
            index: index,
        }))
    }

    fn get_items(
        &self,
        context: &ValidatorContext,
    ) -> Result<Option<Vec<UniqueInParentValueIndexValue>>, HitError> {
        let index = self.index.borrow();
        let parent = context
            .index
            .get_parent(context.id)
            .ok_or(HitError::NoParent())?;
        let items = index
            .index
            .get(context.property, &parent.id, &parent.property);
        match items {
            Some(items) => Ok(Some(items.clone())),
            None => Ok(None),
        }
    }
}

impl Validator<String> for UniqueInParentValidator {
    fn validate(
        &self,
        value: &String,
        context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        let items = self.get_items(context)?;
        match items {
            Some(items) => {
                for item in items.iter() {
                    if item.id != context.id && item.value == Some(value.to_string()) {
                        return Ok(Some(vec![ValidationError::warning(
                            UNIQUE_IN_PARENT.into(),
                            None,
                        )]));
                    }
                }
            }
            None => {}
        }
        Ok(None)
    }

    fn on_kernel_init(&mut self, field_name: &str, model_name: &str) -> Result<(), HitError> {
        self.index
            .borrow_mut()
            .property_names
            .get_or_insert(field_name.to_string());
        self.index
            .borrow_mut()
            .model_names
            .get_or_insert(model_name.to_string());
        Ok(())
    }
}
