use cucumber::{given, then};
use the_ray_tracer_challenge::{Color, Material, Pattern, ReflectionValue, StripePattern};

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
#[given(regex = r"^(\w+)\.ambient ← ([+-]?\d+(?:\.\d+)?)$")]
fn ambient_of_material_is_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_mut_material(&material);
    material.ambient = ReflectionValue::new(value);
}

// material.specular ← 1.0
#[given(regex = r"^(\w+)\.specular ← ([+-]?\d+(?:\.\d+)?)$")]
fn specular_of_material_is_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_mut_material(&material);
    material.specular = ReflectionValue::new(value);
}

// material.diffuse ← 1.0
#[given(regex = r"^(\w+)\.diffuse ← ([+-]?\d+(?:\.\d+)?)$")]
fn diffuse_of_material_is_value(world: &mut RayTracerWorld, material: String, value: f32) {
    let material = world.get_mut_material(&material);
    material.diffuse = ReflectionValue::new(value);
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

    let new_ambient = ReflectionValue::new(value);
    let shape = scene_world
        .elements
        .iter_mut()
        .find(|shape| shape.id() == &uuid)
        .expect("Shape not found");
    let mut new_material = shape.material().clone();
    new_material.ambient = new_ambient;
    shape.set_material(new_material);
}

#[given(
    expr = "{word}.pattern ← stripe_pattern\\(color\\({float}, {float}, {float}), color\\({float}, {float}, {float}))"
)]
fn material_pattern_is_stripe_pattern(
    world: &mut RayTracerWorld,
    material: String,
    color1_red: f32,
    color1_green: f32,
    color1_blue: f32,
    color2_red: f32,
    color2_green: f32,
    color2_blue: f32,
) {
    let material = world.get_mut_material(&material);
    material.pattern = Some(Pattern::SripePattern(StripePattern::new(
        Color::new_color(color1_red, color1_green, color1_blue),
        Color::new_color(color2_red, color2_green, color2_blue),
    )));
}

#[then(expr = "{word} = material\\()")]
fn material_equals_default_material(world: &mut RayTracerWorld, material: String) {
    let actual = world.get_material(&material);
    assert_eq!(actual, &Material::default());
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
    regex = r"^(\w+) = color\(([+-]?\d+(?:\.\d+)?), ([+-]?\d+(?:\.\d+)?), ([+-]?\d+(?:\.\d+)?)\)$"
)]
fn color_equals_color(world: &mut RayTracerWorld, color: String, red: f32, green: f32, blue: f32) {
    let actual = world.get_color(&color);
    let expected = Color::new_color(red, green, blue);
    assert_eq!(actual, &expected);
}
