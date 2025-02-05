use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use uuid::Uuid;
use crate::error::DynastyError;

#[derive(Clone)]
pub struct ClassInfo {
    pub id: Uuid,
    pub name: &'static str,
    pub parent: Option<TypeId>,
    pub type_id: TypeId,
    #[cfg(feature = "reflection")]
    pub reflection_data: crate::reflection::ReflectionData,
}

static CLASS_REGISTRY: Lazy<RwLock<HashMap<TypeId, ClassInfo>>> = 
    Lazy::new(|| RwLock::new(HashMap::new()));

pub struct Registry;

impl Registry {
    pub fn register(info: ClassInfo) {
        let mut registry = CLASS_REGISTRY.write().unwrap();
        registry.insert(info.type_id, info);
    }

    pub fn get(type_id: TypeId) -> Option<ClassInfo> {
        let registry = CLASS_REGISTRY.read().unwrap();
        registry.get(&type_id).cloned()
    }

    pub fn get_by_name(name: &str) -> Option<ClassInfo> {
        let registry = CLASS_REGISTRY.read().unwrap();
        registry.values()
            .find(|info| info.name == name)
            .cloned()
    }

    pub fn is_subclass_of(child: TypeId, parent: TypeId) -> bool {
        let registry = CLASS_REGISTRY.read().unwrap();
        let mut current = child;
        
        while let Some(info) = registry.get(&current) {
            if let Some(parent_id) = info.parent {
                if parent_id == parent {
                    return true;
                }
                current = parent_id;
            } else {
                break;
            }
        }
        false
    }
}