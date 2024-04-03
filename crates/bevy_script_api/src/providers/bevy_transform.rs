#![allow(clippy::all, unused_imports, deprecated, dead_code)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use super::bevy_core::*;
use super::bevy_hierarchy::*;
extern crate self as bevy_script_api;
use bevy_script_api::{lua::RegisterForeignLuaType, ReflectedValue};
/// Describe the position of an entity relative to the reference frame.
/// * To place or move an entity, you should set its [`Transform`].
/// * [`GlobalTransform`] is fully managed by bevy, you cannot mutate it, use
///   [`Transform`] instead.
/// * To get the global transform of an entity, you should get its [`GlobalTransform`].
/// * For transform hierarchies to work correctly, you must have both a [`Transform`] and a [`GlobalTransform`].
///   * You may use the [`TransformBundle`](crate::TransformBundle) to guarantee this.
/// ## [`Transform`] and [`GlobalTransform`]
/// [`Transform`] is the position of an entity relative to its parent position, or the reference
/// frame if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
/// [`GlobalTransform`] is the position of an entity relative to the reference frame.
/// [`GlobalTransform`] is updated from [`Transform`] by systems in the system set
/// [`TransformPropagate`](crate::TransformSystem::TransformPropagate).
/// This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
/// update the [`Transform`] of an entity in this schedule or after, you will notice a 1 frame lag
/// before the [`GlobalTransform`] is updated.
/// # Examples
/// - [`transform`]
/// [`transform`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::transform::components::GlobalTransform",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(
        self,
        #[proxy]
        global_transform: bevy::transform::components::GlobalTransform,
    ) -> bevy::transform::components::GlobalTransform;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::transform::components::GlobalTransform;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &components::global_transform::GlobalTransform) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(
        self,
        #[proxy]
        transform: bevy::transform::components::Transform,
    ) -> bevy::transform::components::GlobalTransform;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] value: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(kind = "Function", output(proxy))]
    fn from_xyz(x: f32, y: f32, z: f32) -> bevy::transform::components::GlobalTransform;

"#,
    r#"

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::transform::components::GlobalTransform;

"#,
    r#"

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation(
        #[proxy]
        rotation: bevy::math::Quat,
    ) -> bevy::transform::components::GlobalTransform;

"#,
    r#"

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(
        #[proxy]
        scale: bevy::math::Vec3,
    ) -> bevy::transform::components::GlobalTransform;

"#,
    r#"
/// Returns the 3d affine transformation matrix as a [`Mat4`].

    #[lua(kind = "Method", output(proxy))]
    fn compute_matrix(&self) -> bevy::math::Mat4;

"#,
    r#"
/// Returns the 3d affine transformation matrix as an [`Affine3A`].

    #[lua(kind = "Method", output(proxy))]
    fn affine(&self) -> bevy::math::Affine3A;

"#,
    r#"
/// Returns the transformation as a [`Transform`].
/// The transform is expected to be non-degenerate and without shearing, or the output
/// will be invalid.

    #[lua(kind = "Method", output(proxy))]
    fn compute_transform(&self) -> bevy::transform::components::Transform;

"#,
    r#"
/// Returns the [`Transform`] `self` would have if it was a child of an entity
/// with the `parent` [`GlobalTransform`].
/// This is useful if you want to "reparent" an [`Entity`](bevy_ecs::entity::Entity).
/// Say you have an entity `e1` that you want to turn into a child of `e2`,
/// but you want `e1` to keep the same global transform, even after re-parenting. You would use:
/// ```
/// # use bevy_transform::prelude::{GlobalTransform, Transform};
/// # use bevy_ecs::prelude::{Entity, Query, Component, Commands};
/// # use bevy_hierarchy::{prelude::Parent, BuildChildren};
/// #[derive(Component)]
/// struct ToReparent {
///     new_parent: Entity,
/// }
/// fn reparent_system(
///     mut commands: Commands,
///     mut targets: Query<(&mut Transform, Entity, &GlobalTransform, &ToReparent)>,
///     transforms: Query<&GlobalTransform>,
/// ) {
///     for (mut transform, entity, initial, to_reparent) in targets.iter_mut() {
///         if let Ok(parent_transform) = transforms.get(to_reparent.new_parent) {
///             *transform = initial.reparented_to(parent_transform);
///             commands.entity(entity)
///                 .remove::<ToReparent>()
///                 .set_parent(to_reparent.new_parent);
///         }
///     }
/// }
/// ```
/// The transform is expected to be non-degenerate and without shearing, or the output
/// will be invalid.

    #[lua(kind = "Method", output(proxy))]
    fn reparented_to(
        &self,
        #[proxy]
        parent: &components::global_transform::GlobalTransform,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
///Return the local right vector (X).

    #[lua(kind = "Method", output(proxy))]
    fn right(&self) -> bevy::math::Vec3;

"#,
    r#"
///Return the local left vector (-X).

    #[lua(kind = "Method", output(proxy))]
    fn left(&self) -> bevy::math::Vec3;

"#,
    r#"
///Return the local up vector (Y).

    #[lua(kind = "Method", output(proxy))]
    fn up(&self) -> bevy::math::Vec3;

"#,
    r#"
///Return the local down vector (-Y).

    #[lua(kind = "Method", output(proxy))]
    fn down(&self) -> bevy::math::Vec3;

"#,
    r#"
///Return the local back vector (Z).

    #[lua(kind = "Method", output(proxy))]
    fn back(&self) -> bevy::math::Vec3;

"#,
    r#"
///Return the local forward vector (-Z).

    #[lua(kind = "Method", output(proxy))]
    fn forward(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Get the translation as a [`Vec3`].

    #[lua(kind = "Method", output(proxy))]
    fn translation(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Get the translation as a [`Vec3A`].

    #[lua(kind = "Method", output(proxy))]
    fn translation_vec3a(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Get an upper bound of the radius from the given `extents`.

    #[lua(kind = "Method")]
    fn radius_vec3a(&self, #[proxy] extents: bevy::math::Vec3A) -> f32;

"#,
    r#"
/// Transforms the given `point`, applying shear, scale, rotation and translation.
/// This moves `point` into the local space of this [`GlobalTransform`].

    #[lua(kind = "Method", output(proxy))]
    fn transform_point(&self, #[proxy] point: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Multiplies `self` with `transform` component by component, returning the
/// resulting [`GlobalTransform`]

    #[lua(kind = "Method", output(proxy))]
    fn mul_transform(
        &self,
        #[proxy]
        transform: bevy::transform::components::Transform,
    ) -> bevy::transform::components::GlobalTransform;

"#]
)]
pub struct GlobalTransform();
/// Describe the position of an entity. If the entity has a parent, the position is relative
/// to its parent position.
/// * To place or move an entity, you should set its [`Transform`].
/// * To get the global transform of an entity, you should get its [`GlobalTransform`].
/// * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
///   * You may use the [`TransformBundle`](crate::TransformBundle) to guarantee this.
/// ## [`Transform`] and [`GlobalTransform`]
/// [`Transform`] is the position of an entity relative to its parent position, or the reference
/// frame if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
/// [`GlobalTransform`] is the position of an entity relative to the reference frame.
/// [`GlobalTransform`] is updated from [`Transform`] by systems in the system set
/// [`TransformPropagate`](crate::TransformSystem::TransformPropagate).
/// This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
/// update the [`Transform`] of an entity during this set or after, you will notice a 1 frame lag
/// before the [`GlobalTransform`] is updated.
/// # Examples
/// - [`transform`]
/// - [`global_vs_local_translation`]
/// [`global_vs_local_translation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/global_vs_local_translation.rs
/// [`transform`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::transform::components::Transform",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::transform::components::Transform;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(
        self,
        #[proxy]
        transform: bevy::transform::components::Transform,
    ) -> bevy::transform::components::Transform;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &components::transform::Transform) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(
        self,
        #[proxy]
        global_transform: bevy::transform::components::GlobalTransform,
    ) -> bevy::transform::components::GlobalTransform;

"#,
    r#"
/// Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
/// is used for z-ordering elements: higher `z`-value will be in front of lower
/// `z`-value.

    #[lua(kind = "Function", output(proxy))]
    fn from_xyz(x: f32, y: f32, z: f32) -> bevy::transform::components::Transform;

"#,
    r#"
/// Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
/// transformation matrix.

    #[lua(kind = "Function", output(proxy))]
    fn from_matrix(
        #[proxy]
        matrix: bevy::math::Mat4,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on
/// all axes.

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on
/// all axes.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation(
        #[proxy]
        rotation: bevy::math::Quat,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on
/// all axes.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(
        #[proxy]
        scale: bevy::math::Vec3,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
/// points towards the `target` position and [`Transform::up`] points towards `up`.
/// In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
/// * if `target` is the same as the transform translation, `Vec3::Z` is used instead
/// * if `up` is zero, `Vec3::Y` is used instead
/// * if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the "right" direction

    #[lua(kind = "Method", output(proxy))]
    fn looking_at(
        self,
        #[proxy]
        target: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Returns this [`Transform`] with a new rotation so that [`Transform::forward`]
/// points in the given `direction` and [`Transform::up`] points towards `up`.
/// In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
/// * if `direction` is zero, `Vec3::Z` is used instead
/// * if `up` is zero, `Vec3::Y` is used instead
/// * if `direction` is parallel with `up`, an orthogonal vector is used as the "right" direction

    #[lua(kind = "Method", output(proxy))]
    fn looking_to(
        self,
        #[proxy]
        direction: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Returns this [`Transform`] with a new translation.

    #[lua(kind = "Method", output(proxy))]
    fn with_translation(
        self,
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Returns this [`Transform`] with a new rotation.

    #[lua(kind = "Method", output(proxy))]
    fn with_rotation(
        self,
        #[proxy]
        rotation: bevy::math::Quat,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Returns this [`Transform`] with a new scale.

    #[lua(kind = "Method", output(proxy))]
    fn with_scale(
        self,
        #[proxy]
        scale: bevy::math::Vec3,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Returns the 3d affine transformation matrix from this transforms translation,
/// rotation, and scale.

    #[lua(kind = "Method", output(proxy))]
    fn compute_matrix(&self) -> bevy::math::Mat4;

"#,
    r#"
/// Returns the 3d affine transformation matrix from this transforms translation,
/// rotation, and scale.

    #[lua(kind = "Method", output(proxy))]
    fn compute_affine(&self) -> bevy::math::Affine3A;

"#,
    r#"
/// Rotates this [`Transform`] by the given rotation.
/// If this [`Transform`] has a parent, the `rotation` is relative to the rotation of the parent.
/// # Examples
/// - [`3d_rotation`]
/// [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs

    #[lua(kind = "MutatingMethod")]
    fn rotate(&mut self, #[proxy] rotation: bevy::math::Quat) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around the given `axis` by `angle` (in radians).
/// If this [`Transform`] has a parent, the `axis` is relative to the rotation of the parent.

    #[lua(kind = "MutatingMethod")]
    fn rotate_axis(&mut self, #[proxy] axis: bevy::math::Vec3, angle: f32) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around the `X` axis by `angle` (in radians).
/// If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.

    #[lua(kind = "MutatingMethod")]
    fn rotate_x(&mut self, angle: f32) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around the `Y` axis by `angle` (in radians).
/// If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.

    #[lua(kind = "MutatingMethod")]
    fn rotate_y(&mut self, angle: f32) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around the `Z` axis by `angle` (in radians).
/// If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.

    #[lua(kind = "MutatingMethod")]
    fn rotate_z(&mut self, angle: f32) -> ();

"#,
    r#"
/// Rotates this [`Transform`] by the given `rotation`.
/// The `rotation` is relative to this [`Transform`]'s current rotation.

    #[lua(kind = "MutatingMethod")]
    fn rotate_local(&mut self, #[proxy] rotation: bevy::math::Quat) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around its local `axis` by `angle` (in radians).

    #[lua(kind = "MutatingMethod")]
    fn rotate_local_axis(&mut self, #[proxy] axis: bevy::math::Vec3, angle: f32) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around its local `X` axis by `angle` (in radians).

    #[lua(kind = "MutatingMethod")]
    fn rotate_local_x(&mut self, angle: f32) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around its local `Y` axis by `angle` (in radians).

    #[lua(kind = "MutatingMethod")]
    fn rotate_local_y(&mut self, angle: f32) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around its local `Z` axis by `angle` (in radians).

    #[lua(kind = "MutatingMethod")]
    fn rotate_local_z(&mut self, angle: f32) -> ();

"#,
    r#"
/// Translates this [`Transform`] around a `point` in space.
/// If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.

    #[lua(kind = "MutatingMethod")]
    fn translate_around(
        &mut self,
        #[proxy]
        point: bevy::math::Vec3,
        #[proxy]
        rotation: bevy::math::Quat,
    ) -> ();

"#,
    r#"
/// Rotates this [`Transform`] around a `point` in space.
/// If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.

    #[lua(kind = "MutatingMethod")]
    fn rotate_around(
        &mut self,
        #[proxy]
        point: bevy::math::Vec3,
        #[proxy]
        rotation: bevy::math::Quat,
    ) -> ();

"#,
    r#"
/// Rotates this [`Transform`] so that [`Transform::forward`] points towards the `target` position,
/// and [`Transform::up`] points towards `up`.
/// In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
/// * if `target` is the same as the transform translation, `Vec3::Z` is used instead
/// * if `up` is zero, `Vec3::Y` is used instead
/// * if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the "right" direction

    #[lua(kind = "MutatingMethod")]
    fn look_at(
        &mut self,
        #[proxy]
        target: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> ();

"#,
    r#"
/// Rotates this [`Transform`] so that [`Transform::forward`] points in the given `direction`
/// and [`Transform::up`] points towards `up`.
/// In some cases it's not possible to construct a rotation. Another axis will be picked in those cases:
/// * if `direction` is zero, `Vec3::NEG_Z` is used instead
/// * if `up` is zero, `Vec3::Y` is used instead
/// * if `direction` is parallel with `up`, an orthogonal vector is used as the "right" direction

    #[lua(kind = "MutatingMethod")]
    fn look_to(
        &mut self,
        #[proxy]
        direction: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> ();

"#,
    r#"
/// Multiplies `self` with `transform` component by component, returning the
/// resulting [`Transform`]

    #[lua(kind = "Method", output(proxy))]
    fn mul_transform(
        &self,
        #[proxy]
        transform: bevy::transform::components::Transform,
    ) -> bevy::transform::components::Transform;

"#,
    r#"
/// Transforms the given `point`, applying scale, rotation and translation.
/// If this [`Transform`] has a parent, this will transform a `point` that is
/// relative to the parent's [`Transform`] into one relative to this [`Transform`].
/// If this [`Transform`] does not have a parent, this will transform a `point`
/// that is in global space into one relative to this [`Transform`].
/// If you want to transform a `point` in global space to the local space of this [`Transform`],
/// consider using [`GlobalTransform::transform_point()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point(&self, #[proxy] point: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns `true` if, and only if, translation, rotation and scale all are
/// finite. If any of them contains a `NaN`, positive or negative infinity,
/// this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] value: bevy::math::Vec3) -> bevy::math::Vec3;

"#]
)]
pub struct Transform {
    #[lua(output(proxy))]
    translation: bevy::math::Vec3,
    #[lua(output(proxy))]
    rotation: bevy::math::Quat,
    #[lua(output(proxy))]
    scale: bevy::math::Vec3,
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
                "GlobalTransform",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaGlobalTransform,
                >::new,
            )?;
        instances
            .add_instance(
                "Transform",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTransform>::new,
            )?;
        Ok(())
    }
}
pub struct BevyTransformAPIProvider;
impl bevy_mod_scripting_core::hosts::APIProvider for BevyTransformAPIProvider {
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
                "BevyTransformAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaGlobalTransform>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGlobalTransform,
                            >,
                        >()
                        .process_type::<LuaTransform>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaTransform,
                            >,
                        >()
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
        app.register_foreign_lua_type::<bevy::transform::components::GlobalTransform>();
        app.register_foreign_lua_type::<bevy::transform::components::Transform>();
    }
}