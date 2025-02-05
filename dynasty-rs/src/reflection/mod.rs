use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::error::DynastyError;

#[cfg(feature = "reflection")]
#[derive(Clone)]
pub struct ReflectionData {
    fields: HashMap<String, FieldInfo>,
    methods: HashMap<String, MethodInfo>,
}

#[cfg(feature = "reflection")]
#[derive(Clone)]
pub struct FieldInfo {
    pub name: String,
    pub type_id: TypeId,
    pub offset: usize,
}

#[cfg(feature = "reflection")]
#[derive(Clone)]
pub struct MethodInfo {
    pub name: String,
    pub signature: String,
}

#[cfg(feature = "reflection")]
pub trait Reflect: crate::traits::Class {
    fn get_field(&self, name: &str) -> Option<&dyn Any>;
    fn get_field_mut(&mut self, name: &str) -> Option<&mut dyn Any>;
    fn set_field(&mut self, name: &str, value: Box<dyn Any>) -> Result<(), DynastyError>;
}

#[cfg(feature = "reflection")]
impl ReflectionData {
    pub fn new<T: 'static>() -> Self {
        Self {
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, name: &str, type_id: TypeId, offset: usize) {
        self.fields.insert(
            name.to_string(),
            FieldInfo {
                name: name.to_string(),
                type_id,
                offset,
            },
        );
    }

    pub fn add_method(&mut self, name: &str, signature: &str) {
        self.methods.insert(
            name.to_string(),
            MethodInfo {
                name: name.to_string(),
                signature: signature.to_string(),
            },
        );
    }
}