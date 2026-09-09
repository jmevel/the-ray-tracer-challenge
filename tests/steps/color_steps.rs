use std::panic;

use cucumber::then;
use the_ray_tracer_challenge::{Color, Object, Tuple, float::float_equals};

use crate::steps::{
    ray_tracer_world::{ElementType, RayTracerWorld},
    tuple_steps::tuple_added_to_tuple_equals_tuple,
};

#[then(expr = "{word} is a point")]
fn is_a_point(world: &mut RayTracerWorld, tuple: String) {
    _ = world.get_point(&tuple);
}

#[then(expr = "{word} is not a point")]
fn is_not_a_point(world: &mut RayTracerWorld, tuple: String) {
    let result = panic::catch_unwind(|| world.get_point(&tuple));
    assert!(result.is_err());
}

#[then(expr = "{word} + {word} = color\\({float}, {float}, {float})")]
fn color_added_to_color_equals_color(
    world: &mut RayTracerWorld,
    tuple1: String,
    tuple2: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    tuple_added_to_tuple_equals_tuple(world, tuple1, tuple2, red, green, blue, 1f32);
}

#[then(expr = "{word} * {word} = color\\({float}, {float}, {float})")]
fn multiplied_color_by_color_equals_color(
    world: &mut RayTracerWorld,
    color1: String,
    scalar_or_color: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    let expected = Color::new_color(red, green, blue);
    let color1 = world.get_color(&color1);

    let result = match scalar_or_color.parse::<f32>() {
        Ok(scalar) => color1 * scalar,
        Err(_) => {
            let color2 = world.get_color(&scalar_or_color);
            color1 * color2
        }
    };

    assert!(float_equals(&result.x(), &expected.x()));
}

#[then(expr = "{word} = {word}.material.color")]
fn color_equals_object_material_color(world: &mut RayTracerWorld, color: String, object: String) {
    let color = world.get_color(&color);
    let object = world.get_element(&object);

    let sphere = match object {
        ElementType::Sphere(sphere) => sphere,
        ElementType::SceneWorldObjectReference((parent_name, uuid)) => {
            let scene_world = world.get_world(parent_name);
            let sphere = scene_world
                .elements
                .iter()
                .find_map(|object| match object {
                    Object::Sphere(sphere) if sphere.id() == uuid => Some(sphere),
                    _ => None,
                })
                .expect("Sphere not found");
            sphere
        }
        _ => panic!("Not implemented"),
    };
    assert_eq!(color, &sphere.material().color);
}
