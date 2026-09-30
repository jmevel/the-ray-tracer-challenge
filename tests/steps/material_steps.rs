use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Color, Material, Object, ReflectionValue};

use crate::steps::ray_tracer_world::{ElementType, RayTracerWorld};

#[given(expr = "{word} ← material\\()")]
fn material_is(world: &mut RayTracerWorld, material_name: String) {
    let material = Material::default();
    world.add_material(material_name, material);
}

#[given(regex = r"^(\w+) ← (true|false)$")]
fn boolean_is(world: &mut RayTracerWorld, boolean_name: String, value: bool) {
    world.add_boolean(boolean_name, value);
}

// material.ambient ← 1.0
#[given(
    regex = r"^(\w+)\.ambient ← ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))$"
)]
fn ambient_of_material_is_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_mut_material(&material);
    material.ambient = ReflectionValue::new(value);
}

#[given(expr = "{word}.material.ambient ← {float}")]
fn ambient_of_element_material_is_value(
    world: &mut RayTracerWorld,
    element_name: String,
    value: f32,
) {
    let (parent_name, uuid) = match world.get_element(&element_name) {
        // values are cloned (becoming owned values instead of references) so the immutable borrow of 'world' can end
        // meaning we then borrow 'world' as mutable later on
        ElementType::SceneWorldObjectReference((parent_name, uuid)) => (parent_name.clone(), *uuid),
        _ => panic!("{element_name} is not a scene world reference"),
    };

    // mutable borrow of 'world' works because the above immutable borrow has ended
    let ElementType::World(scene_world) = world.get_mut_element(&parent_name) else {
        panic!("{parent_name} is not a world");
    };

    let sphere = scene_world
        .elements
        .iter_mut()
        .find_map(|object| match object {
            Object::Sphere(sphere) if sphere.id() == &uuid => Some(sphere),
            _ => None,
        })
        .expect("Sphere not found");

    sphere.material.ambient = ReflectionValue::new(value);
}

#[when(expr = "{word} ← lighting\\({word}, {word}, {word}, {word}, {word}, {word})")]
fn lighting_is(
    world: &mut RayTracerWorld,
    lighting_name: String,
    material: String,
    light_point: String,
    position: String,
    eye_vector: String,
    normal_vector: String,
    in_shadow: String,
) {
    let material = world.get_material(&material);
    let light_point = world.get_point_light(&light_point);
    let position = world.get_point(&position);
    let eye_vector = world.get_vector(&eye_vector);
    let normal_vector = world.get_vector(&normal_vector);
    let in_shadow = world.get_boolean(&in_shadow);

    let lighting = Color::lighting(
        material,
        light_point,
        position,
        eye_vector,
        normal_vector,
        in_shadow,
    );
    world.add_color(lighting_name, lighting);
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
    assert_eq!(material.color, expected);
}

#[then(expr = "{word}.ambient = {float}")]
fn ambient_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(*material.ambient, value);
}

#[then(expr = "{word}.diffuse = {float}")]
fn diffuse_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(*material.diffuse, value);
}

#[then(expr = "{word}.specular = {float}")]
fn specular_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(*material.specular, value);
}

#[then(expr = "{word}.shininess = {float}")]
fn shininess_of_material_equal_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_material(&material);
    assert_eq!(material.shininess, value);
}

// color = color(0.0, 0.0, 0.0)
#[then(
    regex = r"^(\w+) = color\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn color_equals_color(world: &mut RayTracerWorld, color: String, red: f32, green: f32, blue: f32) {
    let actual = world.get_color(&color);
    let expected = Color::new_color(red, green, blue);
    assert_eq!(actual, &expected);
}
