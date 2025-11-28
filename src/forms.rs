use uuid::Uuid;
use std::collections::{HashMap};

#[derive(Debug, PartialEq, Clone)]
pub struct Form {
    pub action: String,
    pub method: String,
    pub fields: HashMap<String, String>
}

#[derive(Debug, PartialEq, Clone)]
pub struct TrackedField {
    id: String,
    origin_form: Form,
    field_name: String
}
impl TrackedField {
    pub fn new(origin_form: Form, origin_name: String) -> Self {
        Self {
            id: Uuid::new_v4().hyphenated().to_string(),
            origin_form: origin_form,
            field_name: origin_name
        }
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}


