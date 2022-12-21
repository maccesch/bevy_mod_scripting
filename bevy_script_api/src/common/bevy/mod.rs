use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::ScriptRef;
/// Common functionality for all script hosts
use bevy::{
    prelude::{
        AppTypeRegistry, Children, Entity, ReflectComponent, ReflectDefault, ReflectResource, World,
    },
    reflect::{
        DynamicArray, DynamicEnum, DynamicList, DynamicMap, DynamicStruct, DynamicTuple,
        DynamicTupleStruct, TypeInfo, TypeRegistration, TypeRegistry,
    },
};
use bevy_mod_scripting_core::{prelude::ScriptError, world::WorldPointer};
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

/// Helper trait for retrieving a world pointer from a script context.
pub trait GetWorld {
    type Error;
    fn get_world(&self) -> Result<WorldPointer, Self::Error>;
}

#[derive(Clone)]
pub struct ScriptTypeRegistration(pub(crate) Arc<TypeRegistration>);

impl ScriptTypeRegistration {
    pub fn new(arc: Arc<TypeRegistration>) -> Self {
        Self(arc)
    }

    #[inline(always)]
    pub fn short_name(&self) -> &str {
        self.0.short_name()
    }

    #[inline(always)]
    pub fn type_name(&self) -> &'static str {
        self.0.type_name()
    }
}

impl std::fmt::Debug for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ScriptTypeRegistration")
            .field(&self.0.type_name())
            .finish()
    }
}

impl std::fmt::Display for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.short_name())
    }
}

impl Deref for ScriptTypeRegistration {
    type Target = TypeRegistration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct ScriptWorld(WorldPointer);

impl std::fmt::Display for ScriptWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("World")
    }
}

impl Deref for ScriptWorld {
    type Target = WorldPointer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ScriptWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<WorldPointer> for ScriptWorld {
    fn as_ref(&self) -> &WorldPointer {
        &self.0
    }
}

impl Into<WorldPointer> for ScriptWorld {
    fn into(self) -> WorldPointer {
        self.0
    }
}

impl ScriptWorld {
    pub fn new(ptr: WorldPointer) -> Self {
        Self(ptr)
    }
    pub fn get_children(&self, parent: Entity) -> Vec<Entity> {
        let w = self.read();
        w.get::<Children>(parent)
            .map(|v| v.to_vec())
            .unwrap_or_default()
    }

    pub fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration> {
        let w = self.read();

        let registry: &AppTypeRegistry = w.get_resource().unwrap();

        let registry = registry.read();

        registry
            .get_with_short_name(type_name)
            .or_else(|| registry.get_with_name(type_name))
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
    }

    pub fn add_default_component(
        &self,
        entity: Entity,
        comp_type: ScriptTypeRegistration,
    ) -> Result<ScriptRef, ScriptError> {
        let mut w = self.write();

        let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
            ScriptError::Other(format!("Not a component {}", comp_type.short_name()))
        })?;

        // this is just a formality
        // TODO: maybe get an add_default impl added to ReflectComponent
        // this means that we don't require ReflectDefault for adding components!
        match comp_type.0.type_info(){
            bevy::reflect::TypeInfo::Struct(_) => component_data.insert(&mut w, entity, &DynamicStruct::default()),
            bevy::reflect::TypeInfo::TupleStruct(_) => component_data.insert(&mut w, entity, &DynamicTupleStruct::default()),
            bevy::reflect::TypeInfo::Tuple(_) => component_data.insert(&mut w, entity, &DynamicTuple::default()),
            bevy::reflect::TypeInfo::List(_) => component_data.insert(&mut w, entity, &DynamicList::default()),
            bevy::reflect::TypeInfo::Array(_) => component_data.insert(&mut w, entity, &DynamicArray::new(Box::new([]))),
            bevy::reflect::TypeInfo::Map(_) => component_data.insert(&mut w, entity, &DynamicMap::default()),
            bevy::reflect::TypeInfo::Value(_) |
            bevy::reflect::TypeInfo::Dynamic(_) => component_data.insert(&mut w, entity,
                comp_type.data::<ReflectDefault>().ok_or_else(||
                    ScriptError::Other(format!("Component {} is a value or dynamic type with no `ReflectDefault` type_data, cannot instantiate sensible value",comp_type.short_name())))?
                    .default()
                    .as_ref()),
            bevy::reflect::TypeInfo::Enum(_) => component_data.insert(&mut w, entity, &DynamicEnum::default())
        };

        Ok(ScriptRef::new_component_ref(
            component_data.clone(),
            entity,
            self.clone().into(),
        ))
    }

    pub fn get_component(
        &self,
        entity: Entity,
        comp_type: ScriptTypeRegistration,
    ) -> Result<Option<ScriptRef>, ScriptError> {
        let w = self.read();

        let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
            ScriptError::Other(format!("Not a component {}", comp_type.short_name()))
        })?;

        Ok(component_data.reflect(&w, entity).map(|_component| {
            ScriptRef::new_component_ref(component_data.clone(), entity, self.clone().into())
        }))
    }
}
