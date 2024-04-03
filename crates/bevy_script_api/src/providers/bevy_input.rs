#![allow(clippy::all, unused_imports, deprecated, dead_code)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
extern crate self as bevy_script_api;
use bevy_script_api::{lua::RegisterForeignLuaType, ReflectedValue};
/// A gamepad with an associated `ID`.
/// ## Usage
/// The primary way to access the individual connected gamepads is done through the [`Gamepads`]
/// `bevy` resource. It is also used inside of [`GamepadConnectionEvent`]s to correspond a gamepad
/// with a connection event.
/// ## Note
/// The `ID` of a gamepad is fixed until the gamepad disconnects or the app is restarted.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::Gamepad",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a new [`Gamepad`].

    #[lua(kind = "Function", output(proxy))]
    fn new(id: usize) -> bevy::input::gamepad::Gamepad;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::Gamepad;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::Gamepad) -> bool;

"#]
)]
pub struct Gamepad {
    id: usize,
}
/// An axis of a [`Gamepad`].
/// ## Usage
/// It is used as the generic `T` value of an [`Axis`] to create `bevy` resources. These
/// resources store the data of the axes of a gamepad and can be accessed inside of a system.
/// ## Updating
/// The gamepad axes resources are updated inside of the [`gamepad_axis_event_system`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadAxis",
    functions[r#"
/// Creates a new [`GamepadAxis`].
/// # Examples
/// ```
/// # use bevy_input::gamepad::{GamepadAxis, GamepadAxisType, Gamepad};
/// #
/// let gamepad_axis = GamepadAxis::new(
///     Gamepad::new(1),
///     GamepadAxisType::LeftStickX,
/// );
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        gamepad: bevy::input::gamepad::Gamepad,
        #[proxy]
        axis_type: bevy::input::gamepad::GamepadAxisType,
    ) -> bevy::input::gamepad::GamepadAxis;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadAxis) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadAxis;

"#]
)]
pub struct GamepadAxis {
    #[lua(output(proxy))]
    gamepad: bevy::input::gamepad::Gamepad,
    #[lua(output(proxy))]
    axis_type: bevy::input::gamepad::GamepadAxisType,
}
/// A type of a [`GamepadAxis`].
/// ## Usage
/// This is used to determine which axis has changed its value when receiving a
/// [`GamepadAxisChangedEvent`]. It is also used in the [`GamepadAxis`]
/// which in turn is used to create the [`Axis<GamepadAxis>`] `bevy` resource.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadAxisType",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadAxisType) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadAxisType;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct GamepadAxisType {}
/// A button of a [`Gamepad`].
/// ## Usage
/// It is used as the generic `T` value of an [`ButtonInput`] and [`Axis`] to create `bevy` resources. These
/// resources store the data of the buttons of a gamepad and can be accessed inside of a system.
/// ## Updating
/// The gamepad button resources are updated inside of the [`gamepad_button_event_system`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadButton",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadButton) -> bool;

"#,
    r#"
/// Creates a new [`GamepadButton`].
/// # Examples
/// ```
/// # use bevy_input::gamepad::{GamepadButton, GamepadButtonType, Gamepad};
/// #
/// let gamepad_button = GamepadButton::new(
///     Gamepad::new(1),
///     GamepadButtonType::South,
/// );
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        gamepad: bevy::input::gamepad::Gamepad,
        #[proxy]
        button_type: bevy::input::gamepad::GamepadButtonType,
    ) -> bevy::input::gamepad::GamepadButton;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadButton;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct GamepadButton {
    #[lua(output(proxy))]
    gamepad: bevy::input::gamepad::Gamepad,
    #[lua(output(proxy))]
    button_type: bevy::input::gamepad::GamepadButtonType,
}
/// A type of a [`GamepadButton`].
/// ## Usage
/// This is used to determine which button has changed its value when receiving a
/// [`GamepadButtonChangedEvent`]. It is also used in the [`GamepadButton`]
/// which in turn is used to create the [`ButtonInput<GamepadButton>`] or
/// [`Axis<GamepadButton>`] `bevy` resources.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadButtonType",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadButtonType) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadButtonType;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct GamepadButtonType {}
/// The key code of a [`KeyboardInput`].
/// ## Usage
/// It is used as the generic `T` value of an [`ButtonInput`] to create a `Res<Input<KeyCode>>`.
/// Code representing the location of a physical key
/// This mostly conforms to the UI Events Specification's [`KeyboardEvent.code`] with a few
/// exceptions:
/// - The keys that the specification calls `MetaLeft` and `MetaRight` are named `SuperLeft` and
///   `SuperRight` here.
/// - The key that the specification calls "Super" is reported as `Unidentified` here.
/// [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
/// ## Updating
/// The resource is updated inside of the [`keyboard_input_system`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::keyboard::KeyCode",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::keyboard::KeyCode;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &keyboard::KeyCode) -> bool;

"#]
)]
pub struct KeyCode {}
/// A button on a mouse device.
/// ## Usage
/// It is used as the generic `T` value of an [`ButtonInput`] to create a `bevy`
/// resource.
/// ## Updating
/// The resource is updated inside of the [`mouse_button_input_system`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::mouse::MouseButton",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::mouse::MouseButton;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &mouse::MouseButton) -> bool;

"#]
)]
pub struct MouseButton {}
/// A touch input event.
/// ## Logic
/// Every time the user touches the screen, a new [`TouchPhase::Started`] event with an unique
/// identifier for the finger is generated. When the finger is lifted, the [`TouchPhase::Ended`]
/// event is generated with the same finger id.
/// After a [`TouchPhase::Started`] event has been emitted, there may be zero or more [`TouchPhase::Moved`]
/// events when the finger is moved or the touch pressure changes.
/// The finger id may be reused by the system after an [`TouchPhase::Ended`] event. The user
/// should assume that a new [`TouchPhase::Started`] event received with the same id has nothing
/// to do with the old finger and is a new finger.
/// A [`TouchPhase::Canceled`] event is emitted when the system has canceled tracking this
/// touch, such as when the window loses focus, or on iOS if the user moves the
/// device against their face.
/// ## Note
/// This event is the translated version of the `WindowEvent::Touch` from the `winit` crate.
/// It is available to the end user and can be used for game logic.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::touch::TouchInput",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &touch::TouchInput) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::touch::TouchInput;

"#]
)]
pub struct TouchInput {
    #[lua(output(proxy))]
    phase: bevy::input::touch::TouchPhase,
    #[lua(output(proxy))]
    position: bevy::math::Vec2,
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    force: ReflectedValue,
    id: u64,
}
/// The logical key code of a [`KeyboardInput`].
/// ## Technical
/// Its values map 1 to 1 to winit's Key.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::keyboard::Key",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &keyboard::Key) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::keyboard::Key;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct Key {}
/// A keyboard input event.
/// This event is the translated version of the `WindowEvent::KeyboardInput` from the `winit` crate.
/// It is available to the end user and can be used for game logic.
/// ## Usage
/// The event is consumed inside of the [`keyboard_input_system`]
/// to update the [`Input<KeyCode>`](ButtonInput<KeyCode>) resource.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::keyboard::KeyboardInput",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::keyboard::KeyboardInput;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &keyboard::KeyboardInput) -> bool;

"#]
)]
pub struct KeyboardInput {
    #[lua(output(proxy))]
    key_code: bevy::input::keyboard::KeyCode,
    #[lua(output(proxy))]
    logical_key: bevy::input::keyboard::Key,
    #[lua(output(proxy))]
    state: bevy::input::ButtonState,
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// Contains the platform-native logical key identifier, known as keysym.
/// Exactly what that means differs from platform to platform, but the values are to some degree
/// tied to the currently active keyboard layout. The same key on the same keyboard may also report
/// different values on different platforms, which is one of the reasons this is a per-platform
/// enum.
/// This enum is primarily used to store raw keysym when Winit doesn't map a given native logical
/// key identifier to a meaningful [`Key`] variant. This lets you use [`Key`], and let the user
/// define keybinds which work in the presence of identifiers we haven't mapped for you yet.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::keyboard::NativeKey",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::keyboard::NativeKey;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &keyboard::NativeKey) -> bool;

"#]
)]
pub struct NativeKey {}
/// Contains the platform-native physical key identifier
/// The exact values vary from platform to platform (which is part of why this is a per-platform
/// enum), but the values are primarily tied to the key's physical location on the keyboard.
/// This enum is primarily used to store raw keycodes when Winit doesn't map a given native
/// physical key identifier to a meaningful [`KeyCode`] variant. In the presence of identifiers we
/// haven't mapped for you yet, this lets you use use [`KeyCode`] to:
/// - Correctly match key press and release events.
/// - On non-web platforms, support assigning keybinds to virtually any key through a UI.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::keyboard::NativeKeyCode",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::keyboard::NativeKeyCode;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &keyboard::NativeKeyCode) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct NativeKeyCode {}
/// A mouse button input event.
/// This event is the translated version of the `WindowEvent::MouseInput` from the `winit` crate.
/// ## Usage
/// The event is read inside of the [`mouse_button_input_system`]
/// to update the [`Input<MouseButton>`](ButtonInput<MouseButton>) resource.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::mouse::MouseButtonInput",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &mouse::MouseButtonInput) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::mouse::MouseButtonInput;

"#]
)]
pub struct MouseButtonInput {
    #[lua(output(proxy))]
    button: bevy::input::mouse::MouseButton,
    #[lua(output(proxy))]
    state: bevy::input::ButtonState,
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// An event reporting the change in physical position of a pointing device.
/// This represents raw, unfiltered physical motion.
/// It is the translated version of [`DeviceEvent::MouseMotion`] from the `winit` crate.
/// All pointing devices connected to a single machine at the same time can emit the event independently.
/// However, the event data does not make it possible to distinguish which device it is referring to.
/// [`DeviceEvent::MouseMotion`]: https://docs.rs/winit/latest/winit/event/enum.DeviceEvent.html#variant.MouseMotion
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::mouse::MouseMotion",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::mouse::MouseMotion;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &mouse::MouseMotion) -> bool;

"#]
)]
pub struct MouseMotion {
    #[lua(output(proxy))]
    delta: bevy::math::Vec2,
}
/// The scroll unit.
/// Describes how a value of a [`MouseWheel`] event has to be interpreted.
/// The value of the event can either be interpreted as the amount of lines or the amount of pixels
/// to scroll.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::mouse::MouseScrollUnit",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &mouse::MouseScrollUnit) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::mouse::MouseScrollUnit;

"#]
)]
pub struct MouseScrollUnit {}
/// A mouse wheel event.
/// This event is the translated version of the `WindowEvent::MouseWheel` from the `winit` crate.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::mouse::MouseWheel",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &mouse::MouseWheel) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::mouse::MouseWheel;

"#]
)]
pub struct MouseWheel {
    #[lua(output(proxy))]
    unit: bevy::input::mouse::MouseScrollUnit,
    x: f32,
    y: f32,
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// A force description of a [`Touch`] input.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::touch::ForceTouch",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &touch::ForceTouch) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::touch::ForceTouch;

"#]
)]
pub struct ForceTouch {}
/// A phase of a [`TouchInput`].
/// ## Usage
/// It is used to describe the phase of the touch input that is currently active.
/// This includes a phase that indicates that a touch input has started or ended,
/// or that a finger has moved. There is also a canceled phase that indicates that
/// the system canceled the tracking of the finger.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::touch::TouchPhase",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &touch::TouchPhase) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::touch::TouchPhase;

"#]
)]
pub struct TouchPhase {}
/// Touchpad magnification event with two-finger pinch gesture.
/// Positive delta values indicate magnification (zooming in) and
/// negative delta values indicate shrinking (zooming out).
/// ## Platform-specific
/// - Only available on **`macOS`**.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::touchpad::TouchpadMagnify",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::touchpad::TouchpadMagnify;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &touchpad::TouchpadMagnify) -> bool;

"#]
)]
pub struct TouchpadMagnify(f32);
/// Touchpad rotation event with two-finger rotation gesture.
/// Positive delta values indicate rotation counterclockwise and
/// negative delta values indicate rotation clockwise.
/// ## Platform-specific
/// - Only available on **`macOS`**.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::touchpad::TouchpadRotate",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &touchpad::TouchpadRotate) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::touchpad::TouchpadRotate;

"#]
)]
pub struct TouchpadRotate(f32);
/// Settings for a [`GamepadAxis`].
/// It is used inside of the [`GamepadSettings`] to define the sensitivity range and
/// threshold for an axis.
/// Values that are higher than `livezone_upperbound` will be rounded up to 1.0.
/// Values that are lower than `livezone_lowerbound` will be rounded down to -1.0.
/// Values that are in-between `deadzone_lowerbound` and `deadzone_upperbound` will be rounded
/// to 0.0.
/// Otherwise, values will not be rounded.
/// The valid range is `[-1.0, 1.0]`.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::AxisSettings",
    functions[r#"
/// Get the value above which inputs will be rounded up to 1.0.

    #[lua(kind = "Method")]
    fn livezone_upperbound(&self) -> f32;

"#,
    r#"
/// Try to set the value above which inputs will be rounded up to 1.0.
/// If the value passed is negative or less than `deadzone_upperbound`,
/// the value will not be changed.
/// Returns the new value of `livezone_upperbound`.

    #[lua(kind = "MutatingMethod")]
    fn set_livezone_upperbound(&mut self, value: f32) -> f32;

"#,
    r#"
/// Get the value below which positive inputs will be rounded down to 0.0.

    #[lua(kind = "Method")]
    fn deadzone_upperbound(&self) -> f32;

"#,
    r#"
/// Try to set the value below which positive inputs will be rounded down to 0.0.
/// If the value passed is negative or greater than `livezone_upperbound`,
/// the value will not be changed.
/// Returns the new value of `deadzone_upperbound`.

    #[lua(kind = "MutatingMethod")]
    fn set_deadzone_upperbound(&mut self, value: f32) -> f32;

"#,
    r#"
/// Get the value below which negative inputs will be rounded down to -1.0.

    #[lua(kind = "Method")]
    fn livezone_lowerbound(&self) -> f32;

"#,
    r#"
/// Try to set the value below which negative inputs will be rounded down to -1.0.
/// If the value passed is positive or greater than `deadzone_lowerbound`,
/// the value will not be changed.
/// Returns the new value of `livezone_lowerbound`.

    #[lua(kind = "MutatingMethod")]
    fn set_livezone_lowerbound(&mut self, value: f32) -> f32;

"#,
    r#"
/// Get the value above which inputs will be rounded up to 0.0.

    #[lua(kind = "Method")]
    fn deadzone_lowerbound(&self) -> f32;

"#,
    r#"
/// Try to set the value above which inputs will be rounded up to 0.0.
/// If the value passed is less than -1.0 or less than `livezone_lowerbound`,
/// the value will not be changed.
/// Returns the new value of `deadzone_lowerbound`.

    #[lua(kind = "MutatingMethod")]
    fn set_deadzone_lowerbound(&mut self, value: f32) -> f32;

"#,
    r#"
/// Get the minimum value by which input must change before the change is registered.

    #[lua(kind = "Method")]
    fn threshold(&self) -> f32;

"#,
    r#"
/// Try to set the minimum value by which input must change before the changes will be applied.
/// If the value passed is not within [0.0..=2.0], the value will not be changed.
/// Returns the new value of threshold.

    #[lua(kind = "MutatingMethod")]
    fn set_threshold(&mut self, value: f32) -> f32;

"#,
    r#"
/// Clamps the `raw_value` according to the `AxisSettings`.

    #[lua(kind = "Method")]
    fn clamp(&self, new_value: f32) -> f32;

"#,
    r#"
/// Filters the `new_value` based on the `old_value`, according to the [`AxisSettings`].
/// Returns the clamped `new_value` if the change exceeds the settings threshold,
/// and `None` otherwise.

    #[lua(kind = "Method")]
    fn filter(
        &self,
        new_value: f32,
        old_value: std::option::Option<f32>,
    ) -> std::option::Option<f32>;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::AxisSettings;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::AxisSettings) -> bool;

"#]
)]
pub struct AxisSettings {}
/// Settings for a [`GamepadButton`].
/// It is used inside of the [`GamepadSettings`] to define the sensitivity range and
/// threshold for a button axis.
/// ## Logic
/// - Values that are higher than or equal to `high` will be rounded to 1.0.
/// - Values that are lower than or equal to `low` will be rounded to 0.0.
/// - Otherwise, values will not be rounded.
/// The valid range is from 0.0 to 1.0, inclusive.
/// ## Updating
/// The current value of a button is received through the [`GamepadButtonChangedEvent`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::ButtonAxisSettings",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::ButtonAxisSettings;

"#,
    r#"
/// Filters the `new_value` based on the `old_value`, according to the [`ButtonAxisSettings`].
/// Returns the clamped `new_value`, according to the [`ButtonAxisSettings`], if the change
/// exceeds the settings threshold, and `None` otherwise.

    #[lua(kind = "Method")]
    fn filter(
        &self,
        new_value: f32,
        old_value: std::option::Option<f32>,
    ) -> std::option::Option<f32>;

"#]
)]
pub struct ButtonAxisSettings {
    high: f32,
    low: f32,
    threshold: f32,
}
/// Manages settings for gamepad buttons.
/// It is used inside of [`GamepadSettings`] to define the threshold for a gamepad button
/// to be considered pressed or released. A button is considered pressed if the `press_threshold`
/// value is surpassed and released if the `release_threshold` value is undercut.
/// Allowed values: `0.0 <= ``release_threshold`` <= ``press_threshold`` <= 1.0`
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::ButtonSettings",
    functions[r#"
/// Returns `true` if the button is pressed.
/// A button is considered pressed if the `value` passed is greater than or equal to the press threshold.

    #[lua(kind = "Method")]
    fn is_pressed(&self, value: f32) -> bool;

"#,
    r#"
/// Returns `true` if the button is released.
/// A button is considered released if the `value` passed is lower than or equal to the release threshold.

    #[lua(kind = "Method")]
    fn is_released(&self, value: f32) -> bool;

"#,
    r#"
/// Get the button input threshold above which the button is considered pressed.

    #[lua(kind = "Method")]
    fn press_threshold(&self) -> f32;

"#,
    r#"
/// Try to set the button input threshold above which the button is considered pressed.
/// If the value passed is outside the range [release threshold..=1.0], the value will not be changed.
/// Returns the new value of the press threshold.

    #[lua(kind = "MutatingMethod")]
    fn set_press_threshold(&mut self, value: f32) -> f32;

"#,
    r#"
/// Get the button input threshold below which the button is considered released.

    #[lua(kind = "Method")]
    fn release_threshold(&self) -> f32;

"#,
    r#"
/// Try to set the button input threshold below which the button is considered released. If the
/// value passed is outside the range [0.0..=press threshold], the value will not be changed.
/// Returns the new value of the release threshold.

    #[lua(kind = "MutatingMethod")]
    fn set_release_threshold(&mut self, value: f32) -> f32;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::ButtonSettings;

"#]
)]
pub struct ButtonSettings {}
/// Gamepad event for when the "value" on the axis changes
/// by an amount larger than the threshold defined in [`GamepadSettings`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadAxisChangedEvent",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadAxisChangedEvent;

"#,
    r#"
/// Creates a [`GamepadAxisChangedEvent`].

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        gamepad: bevy::input::gamepad::Gamepad,
        #[proxy]
        axis_type: bevy::input::gamepad::GamepadAxisType,
        value: f32,
    ) -> bevy::input::gamepad::GamepadAxisChangedEvent;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadAxisChangedEvent) -> bool;

"#]
)]
pub struct GamepadAxisChangedEvent {
    #[lua(output(proxy))]
    gamepad: bevy::input::gamepad::Gamepad,
    #[lua(output(proxy))]
    axis_type: bevy::input::gamepad::GamepadAxisType,
    value: f32,
}
/// Gamepad event for when the "value" (amount of pressure) on the button
/// changes by an amount larger than the threshold defined in [`GamepadSettings`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadButtonChangedEvent",
    functions[r#"
/// Creates a [`GamepadButtonChangedEvent`].

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        gamepad: bevy::input::gamepad::Gamepad,
        #[proxy]
        button_type: bevy::input::gamepad::GamepadButtonType,
        value: f32,
    ) -> bevy::input::gamepad::GamepadButtonChangedEvent;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadButtonChangedEvent) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadButtonChangedEvent;

"#]
)]
pub struct GamepadButtonChangedEvent {
    #[lua(output(proxy))]
    gamepad: bevy::input::gamepad::Gamepad,
    #[lua(output(proxy))]
    button_type: bevy::input::gamepad::GamepadButtonType,
    value: f32,
}
/// A gamepad button input event.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadButtonInput",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadButtonInput;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadButtonInput) -> bool;

"#]
)]
pub struct GamepadButtonInput {
    #[lua(output(proxy))]
    button: bevy::input::gamepad::GamepadButton,
    #[lua(output(proxy))]
    state: bevy::input::ButtonState,
}
/// The connection status of a gamepad.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadConnection",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadConnection;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadConnection) -> bool;

"#]
)]
pub struct GamepadConnection {}
/// A Gamepad connection event. Created when a connection to a gamepad
/// is established and when a gamepad is disconnected.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadConnectionEvent",
    functions[r#"
/// Creates a [`GamepadConnectionEvent`].

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        gamepad: bevy::input::gamepad::Gamepad,
        #[proxy]
        connection: bevy::input::gamepad::GamepadConnection,
    ) -> bevy::input::gamepad::GamepadConnectionEvent;

"#,
    r#"
/// Is the gamepad connected?

    #[lua(kind = "Method")]
    fn connected(&self) -> bool;

"#,
    r#"
/// Is the gamepad disconnected?

    #[lua(kind = "Method")]
    fn disconnected(&self) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadConnectionEvent;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadConnectionEvent) -> bool;

"#]
)]
pub struct GamepadConnectionEvent {
    #[lua(output(proxy))]
    gamepad: bevy::input::gamepad::Gamepad,
    #[lua(output(proxy))]
    connection: bevy::input::gamepad::GamepadConnection,
}
/// A gamepad event.
/// This event type is used over the [`GamepadConnectionEvent`],
/// [`GamepadButtonChangedEvent`] and [`GamepadAxisChangedEvent`] when
/// the in-frame relative ordering of events is important.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadEvent",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadEvent;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadEvent) -> bool;

"#]
)]
pub struct GamepadEvent {}
/// Settings for all [`Gamepad`]s.
/// ## Usage
/// It is used to create a `bevy` resource that stores the settings of every [`GamepadButton`] and
/// [`GamepadAxis`]. If no user defined [`ButtonSettings`], [`AxisSettings`], or [`ButtonAxisSettings`]
/// are defined, the default settings of each are used as a fallback accordingly.
/// ## Note
/// The [`GamepadSettings`] are used inside of `bevy_gilrs` to determine when raw gamepad events from `gilrs`,
/// should register as a [`GamepadEvent`]. Events that don't meet the change thresholds defined in [`GamepadSettings`]
/// will not register. To modify these settings, mutate the corresponding resource.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(derive(debug), remote = "bevy::input::gamepad::GamepadSettings", functions[])]
pub struct GamepadSettings {
    #[lua(output(proxy))]
    default_button_settings: bevy::input::gamepad::ButtonSettings,
    #[lua(output(proxy))]
    default_axis_settings: bevy::input::gamepad::AxisSettings,
    #[lua(output(proxy))]
    default_button_axis_settings: bevy::input::gamepad::ButtonAxisSettings,
    button_settings: ReflectedValue,
    axis_settings: ReflectedValue,
    button_axis_settings: ReflectedValue,
}
/// The current "press" state of an element
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::ButtonState",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &ButtonState) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::ButtonState;

"#,
    r#"
/// Is this button pressed?

    #[lua(kind = "Method")]
    fn is_pressed(&self) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct ButtonState {}
/// Metadata associated with a [`Gamepad`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::input::gamepad::GamepadInfo",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::input::gamepad::GamepadInfo;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &gamepad::GamepadInfo) -> bool;

"#]
)]
pub struct GamepadInfo {
    name: std::string::String,
}
#[derive(Default)]
pub(crate) struct Globals;
impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<
        'lua,
        T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>,
    >(self, instances: &mut T) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances
            .add_instance(
                "Gamepad",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaGamepad>::new,
            )?;
        instances
            .add_instance(
                "GamepadAxis",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaGamepadAxis>::new,
            )?;
        instances
            .add_instance(
                "GamepadButton",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaGamepadButton,
                >::new,
            )?;
        instances
            .add_instance(
                "GamepadAxisChangedEvent",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaGamepadAxisChangedEvent,
                >::new,
            )?;
        instances
            .add_instance(
                "GamepadButtonChangedEvent",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaGamepadButtonChangedEvent,
                >::new,
            )?;
        instances
            .add_instance(
                "GamepadConnectionEvent",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaGamepadConnectionEvent,
                >::new,
            )?;
        Ok(())
    }
}
pub struct BevyInputAPIProvider;
impl bevy_mod_scripting_core::hosts::APIProvider for BevyInputAPIProvider {
    type APITarget = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = bevy_mod_scripting_lua::docs::LuaDocFragment;
    fn attach_api(
        &mut self,
        ctx: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx.get_mut().expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(
                e.to_string(),
            ))
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(
            bevy_mod_scripting_lua::docs::LuaDocFragment::new(
                "BevyInputAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaGamepad>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaGamepad>,
                        >()
                        .process_type::<LuaGamepadAxis>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGamepadAxis,
                            >,
                        >()
                        .process_type::<LuaGamepadAxisType>()
                        .process_type::<LuaGamepadButton>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGamepadButton,
                            >,
                        >()
                        .process_type::<LuaGamepadButtonType>()
                        .process_type::<LuaKeyCode>()
                        .process_type::<LuaMouseButton>()
                        .process_type::<LuaTouchInput>()
                        .process_type::<LuaKey>()
                        .process_type::<LuaKeyboardInput>()
                        .process_type::<LuaNativeKey>()
                        .process_type::<LuaNativeKeyCode>()
                        .process_type::<LuaMouseButtonInput>()
                        .process_type::<LuaMouseMotion>()
                        .process_type::<LuaMouseScrollUnit>()
                        .process_type::<LuaMouseWheel>()
                        .process_type::<LuaForceTouch>()
                        .process_type::<LuaTouchPhase>()
                        .process_type::<LuaTouchpadMagnify>()
                        .process_type::<LuaTouchpadRotate>()
                        .process_type::<LuaAxisSettings>()
                        .process_type::<LuaButtonAxisSettings>()
                        .process_type::<LuaButtonSettings>()
                        .process_type::<LuaGamepadAxisChangedEvent>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGamepadAxisChangedEvent,
                            >,
                        >()
                        .process_type::<LuaGamepadButtonChangedEvent>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGamepadButtonChangedEvent,
                            >,
                        >()
                        .process_type::<LuaGamepadButtonInput>()
                        .process_type::<LuaGamepadConnection>()
                        .process_type::<LuaGamepadConnectionEvent>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGamepadConnectionEvent,
                            >,
                        >()
                        .process_type::<LuaGamepadEvent>()
                        .process_type::<LuaGamepadSettings>()
                        .process_type::<LuaButtonState>()
                        .process_type::<LuaGamepadInfo>()
                },
            ),
        )
    }
    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<bevy::input::gamepad::Gamepad>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadAxis>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadAxisType>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadButton>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadButtonType>();
        app.register_foreign_lua_type::<bevy::input::keyboard::KeyCode>();
        app.register_foreign_lua_type::<bevy::input::mouse::MouseButton>();
        app.register_foreign_lua_type::<bevy::input::touch::TouchInput>();
        app.register_foreign_lua_type::<bevy::input::keyboard::Key>();
        app.register_foreign_lua_type::<bevy::input::keyboard::KeyboardInput>();
        app.register_foreign_lua_type::<bevy::input::keyboard::NativeKey>();
        app.register_foreign_lua_type::<bevy::input::keyboard::NativeKeyCode>();
        app.register_foreign_lua_type::<bevy::input::mouse::MouseButtonInput>();
        app.register_foreign_lua_type::<bevy::input::mouse::MouseMotion>();
        app.register_foreign_lua_type::<bevy::input::mouse::MouseScrollUnit>();
        app.register_foreign_lua_type::<bevy::input::mouse::MouseWheel>();
        app.register_foreign_lua_type::<bevy::input::touch::ForceTouch>();
        app.register_foreign_lua_type::<bevy::input::touch::TouchPhase>();
        app.register_foreign_lua_type::<bevy::input::touchpad::TouchpadMagnify>();
        app.register_foreign_lua_type::<bevy::input::touchpad::TouchpadRotate>();
        app.register_foreign_lua_type::<bevy::input::gamepad::AxisSettings>();
        app.register_foreign_lua_type::<bevy::input::gamepad::ButtonAxisSettings>();
        app.register_foreign_lua_type::<bevy::input::gamepad::ButtonSettings>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadAxisChangedEvent>();
        app.register_foreign_lua_type::<
                bevy::input::gamepad::GamepadButtonChangedEvent,
            >();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadButtonInput>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadConnection>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadConnectionEvent>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadEvent>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadSettings>();
        app.register_foreign_lua_type::<bevy::input::ButtonState>();
        app.register_foreign_lua_type::<bevy::input::gamepad::GamepadInfo>();
    }
}