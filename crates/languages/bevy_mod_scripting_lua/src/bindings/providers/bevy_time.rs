// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use bevy_mod_scripting_core::{
    AddContextInitializer, StoreDocumentation, bindings::ReflectReference,
};
use crate::{
    bindings::proxy::{
        LuaReflectRefProxy, LuaReflectRefMutProxy, LuaReflectValProxy, LuaValProxy,
        LuaIdentityProxy,
    },
    RegisterLua, tealr::mlu::mlua::IntoLua,
};
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    remote = "bevy::time::prelude::Fixed",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::time::prelude::Fixed>,
    ) -> LuaReflectValProxy<bevy::time::prelude::Fixed>;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Fixed {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    remote = "bevy::time::prelude::Real",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::time::prelude::Real>,
    ) -> LuaReflectValProxy<bevy::time::prelude::Real>;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Real {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    remote = "bevy::time::prelude::Timer",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::time::prelude::Timer>,
    ) -> LuaReflectValProxy<bevy::time::prelude::Timer>;

"#,
    r#"
/// Creates a new timer with a given duration in seconds.
/// # Example
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// ```

    #[lua()]
    fn from_seconds(
        duration: f32,
        mode: LuaReflectValProxy<bevy::time::prelude::TimerMode>,
    ) -> LuaReflectValProxy<bevy::time::prelude::Timer>;

"#,
    r#"
/// Returns `true` if the timer has reached its duration.
/// For repeating timers, this method behaves identically to [`Timer::just_finished`].
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer_once = Timer::from_seconds(1.0, TimerMode::Once);
/// timer_once.tick(Duration::from_secs_f32(1.5));
/// assert!(timer_once.finished());
/// timer_once.tick(Duration::from_secs_f32(0.5));
/// assert!(timer_once.finished());
/// let mut timer_repeating = Timer::from_seconds(1.0, TimerMode::Repeating);
/// timer_repeating.tick(Duration::from_secs_f32(1.1));
/// assert!(timer_repeating.finished());
/// timer_repeating.tick(Duration::from_secs_f32(0.8));
/// assert!(!timer_repeating.finished());
/// timer_repeating.tick(Duration::from_secs_f32(0.6));
/// assert!(timer_repeating.finished());
/// ```

    #[lua()]
    fn finished(_self: LuaReflectRefProxy<bevy::time::prelude::Timer>) -> bool;

"#,
    r#"
/// Returns `true` only on the tick the timer reached its duration.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(1.5));
/// assert!(timer.just_finished());
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert!(!timer.just_finished());
/// ```

    #[lua()]
    fn just_finished(_self: LuaReflectRefProxy<bevy::time::prelude::Timer>) -> bool;

"#,
    r#"
/// Returns the time elapsed on the timer as an `f32`.
/// See also [`Timer::elapsed`](Timer::elapsed).

    #[lua()]
    fn elapsed_secs(_self: LuaReflectRefProxy<bevy::time::prelude::Timer>) -> f32;

"#,
    r#"
/// Returns the mode of the timer.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
/// assert_eq!(timer.mode(), TimerMode::Repeating);
/// ```

    #[lua()]
    fn mode(
        _self: LuaReflectRefProxy<bevy::time::prelude::Timer>,
    ) -> LuaReflectValProxy<bevy::time::prelude::TimerMode>;

"#,
    r#"
/// Sets the mode of the timer.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
/// timer.set_mode(TimerMode::Once);
/// assert_eq!(timer.mode(), TimerMode::Once);
/// ```

    #[lua()]
    fn set_mode(
        _self: LuaReflectRefMutProxy<bevy::time::prelude::Timer>,
        mode: LuaReflectValProxy<bevy::time::prelude::TimerMode>,
    ) -> ();

"#,
    r#"
/// Pauses the Timer. Disables the ticking of the timer.
/// See also [`Stopwatch::pause`](Stopwatch::pause).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.pause();
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.elapsed_secs(), 0.0);
/// ```

    #[lua()]
    fn pause(_self: LuaReflectRefMutProxy<bevy::time::prelude::Timer>) -> ();

"#,
    r#"
/// Unpauses the Timer. Resumes the ticking of the timer.
/// See also [`Stopwatch::unpause()`](Stopwatch::unpause).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.pause();
/// timer.tick(Duration::from_secs_f32(0.5));
/// timer.unpause();
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.elapsed_secs(), 0.5);
/// ```

    #[lua()]
    fn unpause(_self: LuaReflectRefMutProxy<bevy::time::prelude::Timer>) -> ();

"#,
    r#"
/// Returns `true` if the timer is paused.
/// See also [`Stopwatch::paused`](Stopwatch::paused).
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// assert!(!timer.paused());
/// timer.pause();
/// assert!(timer.paused());
/// timer.unpause();
/// assert!(!timer.paused());
/// ```

    #[lua()]
    fn paused(_self: LuaReflectRefProxy<bevy::time::prelude::Timer>) -> bool;

"#,
    r#"
/// Resets the timer. The reset doesn't affect the `paused` state of the timer.
/// See also [`Stopwatch::reset`](Stopwatch::reset).
/// Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(1.5));
/// timer.reset();
/// assert!(!timer.finished());
/// assert!(!timer.just_finished());
/// assert_eq!(timer.elapsed_secs(), 0.0);
/// ```

    #[lua()]
    fn reset(_self: LuaReflectRefMutProxy<bevy::time::prelude::Timer>) -> ();

"#,
    r#"
/// Returns the fraction of the timer elapsed time (goes from 0.0 to 1.0).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.fraction(), 0.25);
/// ```

    #[lua()]
    fn fraction(_self: LuaReflectRefProxy<bevy::time::prelude::Timer>) -> f32;

"#,
    r#"
/// Returns the fraction of the timer remaining time (goes from 1.0 to 0.0).
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.fraction_remaining(), 0.75);
/// ```

    #[lua()]
    fn fraction_remaining(_self: LuaReflectRefProxy<bevy::time::prelude::Timer>) -> f32;

"#,
    r#"
/// Returns the remaining time in seconds
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::cmp::Ordering;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
/// timer.tick(Duration::from_secs_f32(0.5));
/// let result = timer.remaining_secs().total_cmp(&1.5);
/// assert_eq!(Ordering::Equal, result);
/// ```

    #[lua()]
    fn remaining_secs(_self: LuaReflectRefProxy<bevy::time::prelude::Timer>) -> f32;

"#,
    r#"
/// Returns the number of times a repeating timer
/// finished during the last [`tick`](Timer<T>::tick) call.
/// For non repeating-timers, this method will only ever
/// return 0 or 1.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
/// timer.tick(Duration::from_secs_f32(6.0));
/// assert_eq!(timer.times_finished_this_tick(), 6);
/// timer.tick(Duration::from_secs_f32(2.0));
/// assert_eq!(timer.times_finished_this_tick(), 2);
/// timer.tick(Duration::from_secs_f32(0.5));
/// assert_eq!(timer.times_finished_this_tick(), 0);
/// ```

    #[lua()]
    fn times_finished_this_tick(
        _self: LuaReflectRefProxy<bevy::time::prelude::Timer>,
    ) -> u32;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(
        _self: LuaReflectRefProxy<bevy::time::prelude::Timer>,
    ) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::time::prelude::Timer>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::time::prelude::Timer>,
        other: LuaReflectRefProxy<bevy::time::prelude::Timer>,
    ) -> bool;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Timer {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    remote = "bevy::time::prelude::TimerMode",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::time::prelude::TimerMode>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::time::prelude::TimerMode>,
        other: LuaReflectRefProxy<bevy::time::prelude::TimerMode>,
    ) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(
        _self: LuaReflectRefProxy<bevy::time::prelude::TimerMode>,
    ) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::time::prelude::TimerMode>,
    ) -> LuaReflectValProxy<bevy::time::prelude::TimerMode>;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct TimerMode {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    remote = "bevy::time::prelude::Virtual",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::time::prelude::Virtual>,
    ) -> LuaReflectValProxy<bevy::time::prelude::Virtual>;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Virtual {}
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    remote = "bevy::time::Stopwatch",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::cmp::PartialEq::<bevy::time::Stopwatch>", composite = "eq")]
    fn eq(
        _self: LuaReflectRefProxy<bevy::time::Stopwatch>,
        other: LuaReflectRefProxy<bevy::time::Stopwatch>,
    ) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::time::Stopwatch>,
    ) -> LuaReflectValProxy<bevy::time::Stopwatch>;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(
        _self: LuaReflectRefProxy<bevy::time::Stopwatch>,
    ) -> ();

"#,
    r#"
/// Create a new unpaused `Stopwatch` with no elapsed time.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let stopwatch = Stopwatch::new();
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// assert_eq!(stopwatch.paused(), false);
/// ```

    #[lua()]
    fn new() -> LuaReflectValProxy<bevy::time::Stopwatch>;

"#,
    r#"
/// Returns the elapsed time since the last [`reset`](Stopwatch::reset)
/// of the stopwatch, in seconds.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.tick(Duration::from_secs(1));
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
/// ```
/// # See Also
/// [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
/// [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.

    #[lua()]
    fn elapsed_secs(_self: LuaReflectRefProxy<bevy::time::Stopwatch>) -> f32;

"#,
    r#"
/// Returns the elapsed time since the last [`reset`](Stopwatch::reset)
/// of the stopwatch, in seconds, as f64.
/// # See Also
/// [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
/// [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` is desirable instead.

    #[lua()]
    fn elapsed_secs_f64(_self: LuaReflectRefProxy<bevy::time::Stopwatch>) -> f64;

"#,
    r#"
/// Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while
/// paused will not have any effect on the elapsed time.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.pause();
/// stopwatch.tick(Duration::from_secs_f32(1.5));
/// assert!(stopwatch.paused());
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// ```

    #[lua()]
    fn pause(_self: LuaReflectRefMutProxy<bevy::time::Stopwatch>) -> ();

"#,
    r#"
/// Unpauses the stopwatch. Resume the effect of ticking on elapsed time.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.pause();
/// stopwatch.tick(Duration::from_secs_f32(1.0));
/// stopwatch.unpause();
/// stopwatch.tick(Duration::from_secs_f32(1.0));
/// assert!(!stopwatch.paused());
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
/// ```

    #[lua()]
    fn unpause(_self: LuaReflectRefMutProxy<bevy::time::Stopwatch>) -> ();

"#,
    r#"
/// Returns `true` if the stopwatch is paused.
/// # Examples
/// ```
/// # use bevy_time::*;
/// let mut stopwatch = Stopwatch::new();
/// assert!(!stopwatch.paused());
/// stopwatch.pause();
/// assert!(stopwatch.paused());
/// stopwatch.unpause();
/// assert!(!stopwatch.paused());
/// ```

    #[lua()]
    fn paused(_self: LuaReflectRefProxy<bevy::time::Stopwatch>) -> bool;

"#,
    r#"
/// Resets the stopwatch. The reset doesn't affect the paused state of the stopwatch.
/// # Examples
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.tick(Duration::from_secs_f32(1.5));
/// stopwatch.reset();
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// ```

    #[lua()]
    fn reset(_self: LuaReflectRefMutProxy<bevy::time::Stopwatch>) -> ();

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Stopwatch {}
#[derive(Default)]
pub(crate) struct Globals;
impl crate::tealr::mlu::ExportInstances for Globals {
    fn add_instances<'lua, T: crate::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> crate::tealr::mlu::mlua::Result<()> {
        instances
            .add_instance("Timer", crate::tealr::mlu::UserDataProxy::<LuaTimer>::new)?;
        instances
            .add_instance(
                "Stopwatch",
                crate::tealr::mlu::UserDataProxy::<LuaStopwatch>::new,
            )?;
        Ok(())
    }
}
fn bevy_time_context_initializer(
    _: &bevy_mod_scripting_core::script::ScriptId,
    ctx: &mut crate::prelude::Lua,
) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
    crate::tealr::mlu::set_global_env(Globals, ctx)?;
    Ok(())
}
pub struct BevyTimeScriptingPlugin;
impl bevy::app::Plugin for BevyTimeScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_lua_proxy::<bevy::time::prelude::Fixed>();
        app.register_lua_proxy::<bevy::time::prelude::Real>();
        app.register_lua_proxy::<bevy::time::prelude::Timer>();
        app.register_lua_proxy::<bevy::time::prelude::TimerMode>();
        app.register_lua_proxy::<bevy::time::prelude::Virtual>();
        app.register_lua_proxy::<bevy::time::Stopwatch>();
        app.add_context_initializer::<()>(bevy_time_context_initializer);
        app.add_documentation_fragment(
            crate::docs::LuaDocumentationFragment::new(
                "BevyTimeAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaFixed>()
                        .process_type::<LuaReal>()
                        .process_type::<LuaTimer>()
                        .process_type::<crate::tealr::mlu::UserDataProxy<LuaTimer>>()
                        .process_type::<LuaTimerMode>()
                        .process_type::<LuaVirtual>()
                        .process_type::<LuaStopwatch>()
                        .process_type::<crate::tealr::mlu::UserDataProxy<LuaStopwatch>>()
                },
            ),
        );
    }
}