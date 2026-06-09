use cucumber::{given, then};
use the_ray_tracer_challenge::{Color, Material, ReflectionValue};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← material\\()")]
fn material_is(world: &mut RayTracerWorld, material_name: String) {
    let material = Material::default();
    world.add_material(material_name, material);
}

#[given(expr = "{word}.ambient ← {float}")]
fn ambient_of_material_is_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_mut_material(&material);
    material.ambient = ReflectionValue::new(value);
}

#[then(expr = "{word}.color = color\\({int}, {int}, {int})")]
fn color_of_material_equals_color(
    world: &mut RayTracerWorld,
    material: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    let expected = Color::new_color(red, green, blue);
    let material = world.get_material(&material);
    assert_eq!(material.color(), &expected);
}

#[then(expr = "{word}.ambient = {float}")]
fn ambient_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(*material.ambient, value);
}

#[then(expr = "{word}.diffuse = {float}")]
fn diffuse_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(material.diffuse(), &value);
}

#[then(expr = "{word}.specular = {float}")]
fn specular_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(material.specular(), &value);
}

#[then(expr = "{word}.shininess = {float}")]
fn shininess_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(material.shininess(), value);
}
