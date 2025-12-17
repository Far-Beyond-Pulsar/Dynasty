use std::any::{Any, TypeId};
use crate::registry::ClassInfo;

pub trait Class: Any {
    fn class_info() -> &'static ClassInfo where Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Inherits<T: Class>: Class {
    fn as_parent(&self) -> &T;
    fn as_parent_mut(&mut self) -> &mut T;
}

pub trait SafeDowncast: Any {}

pub trait Downcast: SafeDowncast {
    // Add Sized bound to Self for the trait methods
    fn is<T: Any>(&self) -> bool 
    where
        Self: Sized
    {
        TypeId::of::<T>() == self.type_id()
    }

    fn downcast_ref<T: Any>(&self) -> Option<&T> 
    where
        Self: Sized
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn SafeDowncast as *const T)) }
        } else {
            None
        }
    }

    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> 
    where
        Self: Sized
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn SafeDowncast as *mut T)) }
        } else {
            None
        }
    }
}

impl dyn SafeDowncast {
    fn is<T: Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn SafeDowncast as *const T)) }
        } else {
            None
        }
    }

    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn SafeDowncast as *mut T)) }
        } else {
            None
        }
    }
}