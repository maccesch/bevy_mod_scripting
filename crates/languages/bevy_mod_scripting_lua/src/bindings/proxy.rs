//! Set of traits used to define how types are turned into and from proxies in Lua.
//! Proxies can either be logical "copies" or owned "direct representations" of the instance, or references to one via the [`bevy_mod_scripting_core::bindings::ReflectReference`] construct.

use bevy::reflect::{FromReflect, Reflect, TypeRegistry};
use bevy_mod_scripting_core::{
    allocator::ReflectAllocator,
    bindings::{ReflectReference, WorldAccessGuard, WorldAccessUnit, WorldAccessWrite},
    error::ReflectionError,
    proxy::{ReflectRefMutProxy, ReflectRefProxy, ReflectValProxy, Unproxy, ValProxy},
};
use tealr::{
    mlu::mlua::{Error, FromLua, IntoLua, Lua, Value},
    ToTypename,
};

/// Local trait alias for the [`Proxied`] trait.
pub trait LuaProxied {
    type Proxy;
}

pub struct LuaValProxy<T: LuaProxied>(pub ValProxy<T, T::Proxy>);
pub struct LuaReflectValProxy<T: LuaProxied>(pub ReflectValProxy<T, T::Proxy>);
pub struct LuaReflectRefProxy<T: LuaProxied>(pub ReflectRefProxy<T, T::Proxy>);
pub struct LuaReflectRefMutProxy<T: LuaProxied>(pub ReflectRefMutProxy<T, T::Proxy>);

macro_rules! impl_lua_unproxy {
    ($ty:ident as $as:ident => $generic:tt : $($bounds:path),* $(| T::Proxy: $($proxy_bounds:tt)*)?) => {
        impl<'w, 'c, $generic:'c> Unproxy<'w, 'c> for $ty<$generic>
        where
            T::Proxy: $($($proxy_bounds)*)?,
            T: $($bounds+)*,
        {
            type Output = <$as<$generic,$generic::Proxy> as Unproxy<'w, 'c>>::Output;

            fn collect_accesses(
                &self,
                guard: &WorldAccessGuard<'w>,
                accesses: &mut bevy::utils::smallvec::SmallVec<[WorldAccessWrite<'w>; 1]>,
            ) -> Result<(), ReflectionError> {
                self.0.collect_accesses(guard, accesses)
            }

            fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
                self.0.unproxy()
            }

            unsafe fn unproxy_with_world(
                &'c mut self,
                guard: &WorldAccessGuard<'w>,
                accesses: &'c [WorldAccessUnit<'w>],
                type_registry: &TypeRegistry,
                allocator: &'c ReflectAllocator,
            ) -> Result<Self::Output, ReflectionError> {
                self.0
                    .unproxy_with_world(guard, accesses, type_registry, allocator)
            }

            fn accesses_len(&self) -> usize {
                self.0.accesses_len()
            }
        }

        impl<'lua, $generic: LuaProxied> FromLua<'lua> for $ty<$generic>
        where
            $generic::Proxy: FromLua<'lua>,
        {
            fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self, Error> {
                let inner: $generic::Proxy = $generic::Proxy::from_lua(value, lua)?;
                let inner = $as::<$generic,$generic::Proxy>::new(inner);
                Ok(Self(inner))
            }
        }

        impl<'lua, $generic: LuaProxied> IntoLua<'lua> for $ty<$generic>
        where
            $generic::Proxy: IntoLua<'lua>,
        {
            fn into_lua(self, lua: &'lua Lua) -> tealr::mlu::mlua::prelude::LuaResult<Value<'lua>> {
                self.0.0.into_lua(lua)
            }
        }

        impl<T: LuaProxied> ToTypename for $ty<T> where T::Proxy: ToTypename {
            fn to_typename() -> tealr::Type {
                T::Proxy::to_typename()
            }
        }
    };
}

macro_rules! impl_lua_proxy {
    ($ty:ident as $as:ident => $generic:tt : $($bounds:path),* $(| T::Proxy: $($proxy_bounds:tt)*)?) => {
        impl<'a,$generic> bevy_mod_scripting_core::proxy::Proxy<'a> for $ty<$generic>
        where
            T::Proxy: $($($proxy_bounds)*)?,
            T: $($bounds+)*,
        {
            type Input=<$as<$generic, $generic::Proxy> as bevy_mod_scripting_core::proxy::Proxy<'a>>::Input;
            fn proxy(value: Self::Input) -> Result<Self, ReflectionError> {
                Ok(Self($as::<$generic,$generic::Proxy>::proxy(value)?))
            }
        }


    };
}

impl_lua_proxy!(LuaValProxy as ValProxy => T : LuaProxied | T::Proxy: From<T>);
impl_lua_proxy!(LuaReflectValProxy as ReflectValProxy => T : LuaProxied,Reflect | T::Proxy: From<ReflectReference> );

impl_lua_unproxy!(LuaValProxy as ValProxy => T : LuaProxied,From<&'c T::Proxy>);
impl_lua_unproxy!(LuaReflectValProxy as ReflectValProxy => T : LuaProxied,FromReflect | T::Proxy: AsRef<ReflectReference>);
impl_lua_unproxy!(LuaReflectRefProxy as ReflectRefProxy => T : LuaProxied,Reflect | T::Proxy: AsRef<ReflectReference>);
impl_lua_unproxy!(LuaReflectRefMutProxy as ReflectRefMutProxy => T: LuaProxied,Reflect | T::Proxy: AsRef<ReflectReference>);

#[cfg(test)]
mod test {

    use std::{cell::UnsafeCell, sync::Arc};

    use bevy::{ecs::component::Component, reflect::Reflect};
    use bevy_mod_scripting_core::{
        allocator::ReflectAllocation,
        bindings::{ReflectBase, ReflectBaseType, ReflectReference},
    };
    use tealr::{
        mlu::mlua::{UserData, UserDataMethods},
        Name,
    };

    use super::*;

    #[derive(Reflect, Component)]
    struct Test(pub String);

    impl Test {
        fn _set(&mut self, value: &Test) {
            self.0 = value.0.clone();
        }
    }

    impl LuaProxied for Test {
        type Proxy = TestProxy;
    }

    #[derive(Debug, Clone)]
    struct TestProxy(ReflectReference);

    impl From<TestProxy> for ReflectReference {
        fn from(value: TestProxy) -> Self {
            value.0
        }
    }

    impl From<ReflectReference> for TestProxy {
        fn from(value: ReflectReference) -> Self {
            TestProxy(value)
        }
    }

    impl AsRef<ReflectReference> for TestProxy {
        fn as_ref(&self) -> &ReflectReference {
            &self.0
        }
    }
    impl<'lua> FromLua<'lua> for TestProxy {
        fn from_lua(
            value: Value<'lua>,
            _lua: &'lua Lua,
        ) -> tealr::mlu::mlua::prelude::LuaResult<Self> {
            match value {
                Value::UserData(ud) => {
                    if let Ok(s) = ud.borrow::<Self>() {
                        Ok(s.clone())
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }
        }
    }

    impl UserData for TestProxy {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("set", |_lua, _self_, _val: LuaReflectRefProxy<Test>| Ok(()))
        }
    }

    impl tealr::ToTypename for TestProxy {
        fn to_typename() -> tealr::Type {
            tealr::Type::Single(tealr::SingleType {
                name: tealr::Name("test".into()),
                kind: tealr::KindOfType::External,
            })
        }
    }

    #[test]
    pub fn test_call_set() {
        let lua = Lua::new();
        let globals = lua.globals();
        let test = Test("test".to_string());
        let mut allocator = ReflectAllocator::default();
        let allocation_id =
            allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(test))));
        let reflect_ref = ReflectReference {
            base: ReflectBaseType {
                type_id: std::any::TypeId::of::<Test>(),
                base_id: ReflectBase::Owned(allocation_id),
            },
            reflect_path: vec![],
        };
        let proxy = TestProxy(reflect_ref);
        globals.set("test", proxy).unwrap();
        lua.load(
            r#"
            test:set(test)
        "#,
        )
        .exec()
        .unwrap();
    }
}
