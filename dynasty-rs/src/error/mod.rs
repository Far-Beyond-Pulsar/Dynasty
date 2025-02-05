use thiserror::Error;
use std::any::TypeId;

#[derive(Error, Debug)]
pub enum DynastyError {
    #[error("Class {0} not found in registry")]
    ClassNotFound(String),
    
    #[error("Invalid inheritance: {0} cannot inherit from {1}")]
    InvalidInheritance(String, String),
    
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        expected: String,
        found: String,
    },
    
    #[error("Reflection error: {0}")]
    ReflectionError(String),

    #[error("Field not found: {0}")]
    FieldNotFound(String),
}