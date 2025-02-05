//! Dynasty: A powerful class inheritance system for Rust game engines

pub mod error;
pub mod registry;
pub mod reflection;
pub mod traits;

// Re-export the proc macros
pub use dynasty_macros::{Class, inherit};

// Re-export core types
pub use crate::registry::ClassInfo;
pub use crate::traits::{Class as ClassTrait, Inherits, SafeDowncast};
#[cfg(feature = "reflection")]
pub use crate::reflection::{Reflect, ReflectionData};

/// Prelude module containing commonly used types and traits
pub mod prelude {
    pub use crate::{Class, inherit};
    pub use crate::traits::{Inherits, SafeDowncast};
    pub use crate::registry::Registry;
    
    #[cfg(feature = "reflection")]
    pub use crate::reflection::Reflect;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};

    // Basic test structs
    #[derive(Class, Debug)]
    struct Entity {
        id: u64,
        name: String,
    }

    #[inherit(Entity)]
    #[derive(Debug)]
    struct Character {
        base: Entity,
        health: f32,
        level: u32,
    }

    #[inherit(Character)]
    #[derive(Debug)]
    struct Player {
        base: Character,
        experience: u32,
    }

    // Test basic class creation and inheritance
    #[test]
    fn test_basic_inheritance() {
        let entity = Entity {
            id: 1,
            name: "Test Entity".to_string(),
        };

        let character = Character {
            base: Entity {
                id: 2,
                name: "Test Character".to_string(),
            },
            health: 100.0,
            level: 1,
        };

        assert_eq!(entity.id, 1);
        assert_eq!(character.base.id, 2);
        assert_eq!(character.health, 100.0);
    }

    // Test multi-level inheritance
    #[test]
    fn test_multilevel_inheritance() {
        let player = Player {
            base: Character {
                base: Entity {
                    id: 3,
                    name: "Test Player".to_string(),
                },
                health: 100.0,
                level: 1,
            },
            experience: 0,
        };

        assert_eq!(player.base.base.id, 3);
        assert_eq!(player.base.health, 100.0);
        assert_eq!(player.experience, 0);
    }

    // Test class registry
    #[test]
    fn test_class_registry() {
        let entity_info = Entity::class_info();
        let character_info = Character::class_info();

        assert_eq!(entity_info.name, "Entity");
        assert_eq!(character_info.name, "Character");
        assert!(character_info.parent.is_some());
    }

    // Test type checking and downcasting
    #[test]
    fn test_type_checking() {
        let player = Player {
            base: Character {
                base: Entity {
                    id: 4,
                    name: "Test Player".to_string(),
                },
                health: 100.0,
                level: 1,
            },
            experience: 0,
        };

        // Test inheritance relationships
        assert_eq!(player.as_any().type_id(), TypeId::of::<Player>());
        assert_eq!(player.base.as_any().type_id(), TypeId::of::<Character>());
        assert_eq!(player.base.base.as_any().type_id(), TypeId::of::<Entity>());
        
        // Test safe downcasting
        let as_character = player.base.as_any().downcast_ref::<Character>();
        assert!(as_character.is_some());
    }

    // Test game patterns
    #[test]
    fn test_game_patterns() {
        #[derive(Class)]
        struct Transform {
            position: (f32, f32, f32),
        }

        #[inherit(Entity)]
        struct GameObject {
            base: Entity,
            transform: Transform,
        }

        let game_object = GameObject {
            base: Entity {
                id: 6,
                name: "Game Object".to_string(),
            },
            transform: Transform {
                position: (0.0, 0.0, 0.0),
            },
        };

        assert_eq!(game_object.transform.position, (0.0, 0.0, 0.0));
    }

    #[cfg(feature = "reflection")]
    mod reflection_tests {
        use super::*;
        use crate::reflection::Reflect;

        impl Reflect for Player {
            fn get_field(&self, name: &str) -> Option<&dyn Any> {
                match name {
                    "experience" => Some(&self.experience),
                    _ => None,
                }
            }

            fn get_field_mut(&mut self, name: &str) -> Option<&mut dyn Any> {
                match name {
                    "experience" => Some(&mut self.experience),
                    _ => None,
                }
            }

            fn set_field(&mut self, name: &str, value: Box<dyn Any>) -> Result<(), error::DynastyError> {
                match name {
                    "experience" => {
                        let type_id = value.type_id();
                        match value.downcast::<u32>() {
                            Ok(exp) => {
                                self.experience = *exp;
                                Ok(())
                            }
                            Err(_) => Err(error::DynastyError::TypeMismatch {
                                expected: "u32".to_string(),
                                found: format!("{:?}", type_id),
                            })
                        }
                    }
                    _ => Err(error::DynastyError::FieldNotFound(name.to_string())),
                }
            }
        }

        #[test]
        fn test_reflection() {
            let player = Player {
                base: Character {
                    base: Entity {
                        id: 5,
                        name: "Reflected Player".to_string(),
                    },
                    health: 100.0,
                    level: 1,
                },
                experience: 100,
            };

            // Test field reflection
            if let Some(exp_field) = player.get_field("experience") {
                if let Some(exp_value) = exp_field.downcast_ref::<u32>() {
                    assert_eq!(*exp_value, 100);
                } else {
                    panic!("Failed to downcast experience field to u32");
                }
            } else {
                panic!("Failed to get experience field");
            }

            // Test non-existent field
            assert!(player.get_field("nonexistent").is_none());
        }
    }
}