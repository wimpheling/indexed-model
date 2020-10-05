use crate::index::IndexEntry;
use crate::model::Model;
use crate::object_data::Id;
use crate::object_data::ObjectValue;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HitEntry {
    pub(in crate) entry: Rc<RefCell<IndexEntry>>,
    pub(in crate) model: Rc<Model>,
}

impl HitEntry {
    pub fn get(&self, property: &str) -> ObjectValue {
        let entry = self.entry.borrow();
        return entry.get(property).clone();
    }

    pub fn get_id(&self) -> Id {
        let entry = self.entry.borrow();
        return entry.get_id().clone();
    }

    pub fn get_model(&self) -> Rc<Model> {
        return self.model.clone();
    }

    pub fn get_parent_id(&self) -> Option<String> {
        let entry = self.entry.borrow();
        return entry.get_parent_id();
    }
}