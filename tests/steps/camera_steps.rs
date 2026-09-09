use std::f32;

use cucumber::{given, then, when};
use the_ray_tracer_challenge::Camera;

use crate::steps::ray_tracer_world::RayTracerWorld;

// v ← 2
#[given(
    regex = r#"^([a-zA-Z0-9]*) ← ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))$"#
)]
fn value_is_integer(world: &mut RayTracerWorld, integer_name: String, value: usize) {
    world.add_integer(integer_name, value);
}

#[given(expr = "{word} ← π\\/{float}")]
fn value_is_float(world: &mut RayTracerWorld, float_name: String, denominator: f32) {
    world.add_float(float_name, f32::consts::PI / denominator);
}

#[when(expr = "{word} ← camera\\({word}, {word}, {word})")]
fn camera_is(
    world: &mut RayTracerWorld,
    camera_name: String,
    hsize: String,
    vsize: String,
    field_of_view: String,
) {
    let hsize = world.get_integer(&hsize);
    let vsize = world.get_integer(&vsize);
    let field_of_view = world.get_float(&field_of_view);

    world.add_camera(camera_name, Camera::new(hsize, vsize, field_of_view));
}

#[then(expr = "{word}.hsize = {int}")]
fn hsize_of_camera_equals_value(world: &mut RayTracerWorld, camera: String, value: usize) {
    let camera = world.get_camera(&camera);
    assert_eq!(camera.hsize(), value);
}

#[then(expr = "{word}.vsize = {int}")]
fn vsize_of_camera_equals_value(world: &mut RayTracerWorld, camera: String, value: usize) {
    let camera = world.get_camera(&camera);
    assert_eq!(camera.vsize(), value);
}

#[then(expr = "{word}.field_of_view = π\\/{int}")]
fn field_of_view_of_camera_equals_value(
    world: &mut RayTracerWorld,
    camera: String,
    denominator: f32,
) {
    let camera = world.get_camera(&camera);
    let expected = f32::consts::PI / denominator;
    assert_eq!(camera.field_of_view(), expected);
}
