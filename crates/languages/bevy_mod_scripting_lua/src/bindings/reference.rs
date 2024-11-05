use std::{any::Any, error::Error};

use bevy::{
    ecs::{reflect::AppTypeRegistry, world::Mut},
    reflect::{OffsetAccess, ParsedPath, ReflectFromReflect},
};
use bevy_mod_scripting_core::{
    bindings::{ReflectAllocator, ReflectReference, Unproxy, WorldCallbackAccess},
    error::ScriptError,
};
use tealr::mlu::{
    mlua::{self, FromLua, IntoLua, Lua, MetaMethod, UserData, Value},
    TealData,
};

use crate::{impl_userdata_from_lua, ReflectLuaProxied, ReflectLuaValue};

use super::{
    proxy::{LuaProxied, LuaValProxy},
    world::GetWorld,
};

/// Lua UserData wrapper for [`bevy_mod_scripting_core::bindings::ReflectReference`].
/// Acts as a lua reflection interface. Any value which is registered in the type registry can be interacted with using this type.
#[derive(Debug, Clone, tealr::mlu::UserData, tealr::ToTypename)]
pub struct LuaReflectReference(pub ReflectReference);

impl LuaReflectReference {
    /// Queries the reflection system for a proxy registration for the underlying type.
    /// If found will convert to lua using this proxy
    /// If not found will use <Self as [`IntoLua`]>::into_lua to convert to lua
    pub fn to_lua_proxy(self, lua: &Lua) -> Result<Value<'_>, mlua::Error> {
        // note we do not need to refer to LuaWorld here, it does not matter what the proxy is, that's pretty neat,
        let world = lua.get_world()?;
        // TODO: i don't like the pingponging between errors here, need something more ergonomic
        let result: Result<Value, ScriptError> =
            world.with_resource(|world, type_registry: Mut<AppTypeRegistry>| {
                world.with_resource(|world, allocator: Mut<ReflectAllocator>| {
                    let type_registry = type_registry.read();
                    // first we need the type id of the pointed to object to figure out how to work with it
                    let type_id =
                        self.0
                            .with_reflect(world, &type_registry, Some(&allocator), |r| r.type_id());
                    if let Some(type_data) = type_registry.get_type_data::<ReflectLuaValue>(type_id)
                    {
                        self.0
                            .with_reflect(world, &type_registry, Some(&allocator), |r| {
                                Ok((type_data.into_value)(r, lua)?)
                            })
                    } else if let Some(type_data) =
                        type_registry.get_type_data::<ReflectLuaProxied>(type_id)
                    {
                        Ok((type_data.into_proxy)(self.0.clone(), lua)?)
                    } else {
                        Ok(self.clone().into_lua(lua)?)
                    }
                })
            });
        result.map_err(mlua::Error::external)
    }

    pub fn set_with_lua_proxy(&self, lua: &Lua, value: Value) -> Result<(), mlua::Error> {
        let world = lua.get_world()?;
        let result: Result<(), ScriptError> =
            world.with_resource(|world, type_registry: Mut<AppTypeRegistry>| {
                world.with_resource(|world, allocator: Mut<ReflectAllocator>| {
                    let type_registry = type_registry.read();
                    let type_id =
                        self.0
                            .with_reflect(world, &type_registry, Some(&allocator), |r| r.type_id());

                    if let Some(type_data) = type_registry.get_type_data::<ReflectLuaValue>(type_id)
                    {
                        self.0
                            .with_reflect_mut(world, &type_registry, Some(&allocator), |r| {
                                Ok((type_data.set_value)(r, value, lua)?)
                            })
                    } else if let Some(type_data) =
                        type_registry.get_type_data::<ReflectLuaProxied>(type_id)
                    {
                        let other = (type_data.from_proxy)(value, lua)?;

                        // first we need to get a copy of the other value
                        let other = other
                            .with_reflect(world, &type_registry, Some(&allocator), |r| {
                                type_registry
                                    .get_type_data::<ReflectFromReflect>(r.type_id())
                                    .and_then(|from_reflect_td| from_reflect_td.from_reflect(r))
                            })
                            .ok_or_else(|| {
                                ScriptError::new_reflection_error(format!(
                                    "Failed to call ReflectFromReflect for type id: {:?}",
                                    type_registry.get_type_info(type_id).map(|t| t.type_path())
                                ))
                            })?;

                        // now we can set it
                        self.0
                            .with_reflect_mut(world, &type_registry, Some(&allocator), |r| {
                                r.set(other).map_err(|e| {
                                    ScriptError::new_runtime_error(format!(
                                        "Invalid assignment `{:?}` = `{:?}`. Wrong type.",
                                        self.0.clone(),
                                        e,
                                    ))
                                })
                            })?;
                        Ok(())
                    } else {
                        Err(ScriptError::new_runtime_error(format!(
                            "Invalid assignment `{:?}` = `{:?}`. Wrong type.",
                            self.0.clone(),
                            value,
                        )))
                    }
                })
            });

        result.map_err(mlua::Error::external)
    }

    /// Adjusts all the numeric accesses in the path from 1-indexed to 0-indexed
    pub fn to_host_index(path: &mut ParsedPath) {
        path.0.iter_mut().for_each(|a| match a.access {
            bevy::reflect::Access::FieldIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::TupleIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::ListIndex(ref mut i) => *i -= 1,
            _ => {}
        });
    }

    /// Adjusts all the numeric accesses in the path from 0-indexed to 1-indexed
    pub fn from_host_index(path: &mut ParsedPath) {
        path.0.iter_mut().for_each(|a| match a.access {
            bevy::reflect::Access::FieldIndex(ref mut i) => *i += 1,
            bevy::reflect::Access::TupleIndex(ref mut i) => *i += 1,
            bevy::reflect::Access::ListIndex(ref mut i) => *i += 1,
            _ => {}
        });
    }

    pub fn parse_value_index(value: Value) -> Result<ParsedPath, mlua::Error> {
        if let Some(num) = value.as_usize() {
            Ok(vec![OffsetAccess {
                access: bevy::reflect::Access::ListIndex(num),
                offset: Some(1),
            }]
            .into())
        } else if let Some(key) = value.as_str() {
            if let Some(tuple_struct_index) = key.strip_prefix("_") {
                if let Ok(index) = tuple_struct_index.parse::<usize>() {
                    return Ok(vec![OffsetAccess {
                        access: bevy::reflect::Access::TupleIndex(index),
                        offset: Some(1),
                    }]
                    .into());
                }
            }

            ParsedPath::parse(key).map_err(|e| mlua::Error::external(e.to_string()))
        } else {
            Err(mlua::Error::external("Invalid index"))
        }
    }
}

impl_userdata_from_lua!(LuaReflectReference);

impl LuaProxied for ReflectReference {
    type Proxy = LuaReflectReference;
}

impl TealData for LuaReflectReference {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |l, (mut self_, key): (LuaReflectReference, Value)| {
                // catchall, parse the path
                let mut elem = Self::parse_value_index(key)?;
                Self::to_host_index(&mut elem);
                self_.0.index_path(elem);
                self_.to_lua_proxy(l)
            },
        );
        m.add_meta_function(
            MetaMethod::NewIndex,
            |l, (mut self_, key, value): (LuaReflectReference, Value, Value)| {
                let mut elem = Self::parse_value_index(key)?;
                Self::to_host_index(&mut elem);
                self_.0.index_path(elem);
                self_.set_with_lua_proxy(l, value)
            },
        );
    }
}

#[cfg(test)]
mod test {

    use bevy::{
        app::App,
        ecs::{reflect::AppTypeRegistry, world::World},
        reflect::{FromReflect, OffsetAccess, Reflect},
    };
    use bevy_mod_scripting_core::{
        bindings::ReflectAllocator,
        bindings::{ReflectBase, ReflectBaseType, WorldAccessGuard, WorldCallbackAccess},
    };
    use bevy_mod_scripting_derive::LuaProxy;

    use crate::{bindings::world::LuaWorld, RegisterLua};

    use super::*;

    #[derive(Reflect)]
    struct TestStruct {
        value: usize,
        proxy: TestProxied,
        proxies: Vec<TestProxied>,
    }

    #[derive(Reflect)]
    struct TestTupleStruct(usize, TestProxied, Vec<TestProxied>);

    #[derive(Reflect)]
    enum TestTupleEnum {
        Value(usize),
        Proxy(TestProxied),
        Proxies(Vec<TestProxied>),
    }

    #[derive(Reflect, LuaProxy)]
    #[proxy(bms_core_path = "bevy_mod_scripting_core", bms_lua_path = "crate")]
    #[reflect(LuaProxied)]
    pub struct TestProxied;

    impl PartialEq for LuaTestProxied {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    /// asserts that setting then indexing into a LuaReflectReference of type T with the given expression returns the expected value.
    /// Provides `t and `world` globals, with t being the LuaReflectReference to the provided value.
    fn assert_lua_set_get_returns<
        T: Reflect,
        F: Fn(ReflectReference) -> O,
        O: for<'l> FromLua<'l> + for<'l> IntoLua<'l> + PartialEq + std::fmt::Debug,
    >(
        world: Option<World>,
        val: T,
        expr: &'static str,
        expected: F,
    ) {
        let mut world = world.unwrap_or_default();
        let lua = Lua::new();
        let mut allocator = ReflectAllocator::default();
        let reflect_ref = LuaReflectReference(ReflectReference::new_allocated(val, &mut allocator));
        world.insert_resource(allocator);

        WorldCallbackAccess::with_callback_access(&mut world, |access| {
            let globals = lua.globals();
            globals.set("test", reflect_ref.clone()).unwrap();
            globals.set("world", LuaWorld(access.clone())).unwrap();
            globals
                .set("expected", expected(reflect_ref.0.clone()))
                .unwrap();

            let lua_code = format!(
                r#"
                {expr} = expected
                return {expr}
                "#
            );
            let result = lua
                .load(&lua_code)
                .into_function()
                .unwrap_or_else(|e| panic!("Could not load lua code into function: `{e}`"))
                .call(())
                .unwrap_or_else(|e| {
                    panic!("Could not convert expression value to expected type: `{e}`")
                });
            let result: O = result;
            assert_eq!(result, expected(reflect_ref.0));
        });
    }

    #[test]
    fn test_index_lua_value() {
        // so we have the registry and can just do this
        let mut app = App::new();
        app.register_lua_value::<usize>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestStruct {
                value: 123,
                proxy: TestProxied,
                proxies: vec![],
            },
            "test.value",
            |_| 123usize,
        );

        let mut app = App::new();
        app.register_lua_value::<usize>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestTupleStruct(123, TestProxied, vec![]),
            "test._1",
            |_| 123usize,
        );

        let mut app = App::new();
        app.register_lua_value::<usize>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestTupleEnum::Value(123usize),
            "test._1",
            |_| 123usize,
        );
    }

    #[test]
    fn test_index_lua_proxy() {
        // so we have the registry and can just do this
        let mut app = App::new();
        app.register_lua_proxy::<TestProxied>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestStruct {
                value: 123,
                proxy: TestProxied,
                proxies: vec![],
            },
            "test.proxy",
            |mut r| {
                r.index_path(ParsedPath::parse_static("proxy").unwrap());
                LuaTestProxied(r)
            },
        );

        let mut app = App::new();
        app.register_lua_proxy::<TestProxied>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestTupleStruct(123, TestProxied, vec![]),
            "test._2",
            |mut r| {
                r.index_path(ParsedPath::parse_static(".1").unwrap());
                LuaTestProxied(r)
            },
        );

        let mut app = App::new();
        app.register_lua_proxy::<TestProxied>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestTupleEnum::Proxy(TestProxied),
            "test._1",
            |mut r| {
                r.index_path(ParsedPath::parse_static(".0").unwrap());
                LuaTestProxied(r)
            },
        );
    }

    #[test]
    fn test_index_lua_proxy_vec() {
        // so we have the registry and can just do this
        let mut app = App::new();
        app.register_lua_proxy::<TestProxied>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestStruct {
                value: 123,
                proxy: TestProxied,
                proxies: vec![TestProxied],
            },
            "test.proxies[1]",
            |mut r| {
                r.index_path(ParsedPath::parse_static("proxies").unwrap());
                r.index_path(ParsedPath::parse_static("[0]").unwrap());
                LuaTestProxied(r)
            },
        );

        let mut app = App::new();
        app.register_lua_proxy::<TestProxied>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestTupleStruct(123, TestProxied, vec![TestProxied]),
            "test._3[1]",
            |mut r| {
                r.index_path(ParsedPath::parse_static(".2").unwrap());
                r.index_path(ParsedPath::parse_static("[0]").unwrap());
                LuaTestProxied(r)
            },
        );

        let mut app = App::new();
        app.register_lua_proxy::<TestProxied>();

        assert_lua_set_get_returns(
            Some(app.world),
            TestTupleEnum::Proxies(vec![TestProxied]),
            "test._1[1]",
            |mut r| {
                r.index_path(ParsedPath::parse_static(".0").unwrap());
                r.index_path(ParsedPath::parse_static("[0]").unwrap());
                LuaTestProxied(r)
            },
        );
    }
}