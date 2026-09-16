use std::f32;

use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Camera, Matrix};

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

#[given(expr = "{word} ← camera\\({int}, {int}, π\\/{int})")]
fn camera_is(
    world: &mut RayTracerWorld,
    camera_name: String,
    hsize: usize,
    vsize: usize,
    field_of_view_denominator: usize,
) {
    let field_of_view = f32::consts::PI / field_of_view_denominator as f32;
    world.add_camera(camera_name, Camera::new(hsize, vsize, field_of_view));
}

#[when(expr = "{word} ← camera\\({word}, {word}, {word})")]
fn camera_is2(
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

#[then(expr = "{word}.pixel_size = {float}")]
fn camera_pixel_size_equals(world: &mut RayTracerWorld, camera: String, expected_pixel_size: f32) {
    let camera = world.get_camera(&camera);
    assert_eq!(camera.pixel_size(), expected_pixel_size);
}

#[when(expr = "{word} ← ray_for_pixel\\({word}, {int}, {int})")]
fn ray_is_ray_for_pixel(
    world: &mut RayTracerWorld,
    ray_name: String,
    camera: String,
    pixel_x: usize,
    pixel_y: usize,
) {
    let camera = world.get_camera(&camera);
    let ray = camera.ray_for_pixel(pixel_x, pixel_y);
    world.add_ray(ray_name, ray);
}

#[when(expr = "{word}.transform ← rotation_y\\(π\\/{int}) * translation\\({int}, {int}, {int})")]
fn camera_transform_is_rotation_y_and_translation(
    world: &mut RayTracerWorld,
    camera: String,
    rotation_y_denominator: f32,
    translation_x: f32,
    translation_y: f32,
    translation_z: f32,
) {
    let camera = world.get_mut_camera(&camera);
    let transformation = Matrix::new_rotation_y(f32::consts::PI / rotation_y_denominator)
        * Matrix::new_translation(translation_x, translation_y, translation_z);
    camera.transform = transformation;
}
