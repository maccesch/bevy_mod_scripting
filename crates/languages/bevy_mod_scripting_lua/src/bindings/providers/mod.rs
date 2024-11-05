// @generated by cargo bevy-api-gen collect, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
pub(crate) mod bevy_ecs;
pub(crate) mod bevy_transform;
pub(crate) mod bevy_input;
pub(crate) mod bevy_core;
pub(crate) mod bevy_time;
pub(crate) mod bevy_hierarchy;
pub(crate) mod bevy_window;
pub(crate) mod bevy_reflect;
pub struct LuaBevyScriptingPlugin;
impl bevy::app::Plugin for LuaBevyScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        bevy_ecs::BevyEcsScriptingPlugin.build(app);
        bevy_transform::BevyTransformScriptingPlugin.build(app);
        bevy_input::BevyInputScriptingPlugin.build(app);
        bevy_core::BevyCoreScriptingPlugin.build(app);
        bevy_time::BevyTimeScriptingPlugin.build(app);
        bevy_hierarchy::BevyHierarchyScriptingPlugin.build(app);
        bevy_window::BevyWindowScriptingPlugin.build(app);
        bevy_reflect::BevyReflectScriptingPlugin.build(app);
    }
}