use std::panic;

use cucumber::{then, when};
use the_ray_tracer_challenge::{Color, Point, Tuple, float::float_equals};

use crate::steps::{
    ray_tracer_world::{ElementType, RayTracerWorld},
    tuple_steps::tuple_added_to_tuple_equals_tuple,
};

#[when(expr = "{word} ← lighting\\({word}, {word}, {word}, {word}, {word}, {word})")]
fn color_is_lighting(
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

// c1 ← lighting(m, light, point(0.9, 0, 0), eyev, normalv, false)
#[when(
    regex = r"^(\w+) ← lighting\((\w+), (\w+), point\(([+-]?\d+(?:\.\d+)?), ([+-]?\d+(?:\.\d+)?), ([+-]?\d+(?:\.\d+)?)\), (\w+), (\w+), (true|false)\)$"
)]
fn color_is_lighting2(
    world: &mut RayTracerWorld,
    color_name: String,
    material: String,
    light_point: String,
    x: f32,
    y: f32,
    z: f32,
    eye_vector: String,
    normal_vector: String,
    in_shadow: bool,
) {
    let material = world.get_material(&material);
    let light_point = world.get_point_light(&light_point);
    let position = &Point::new_point(x, y, z);
    let eye_vector = world.get_vector(&eye_vector);
    let normal_vector = world.get_vector(&normal_vector);

    let lighting = Color::lighting(
        material,
        light_point,
        position,
        eye_vector,
        normal_vector,
        in_shadow,
    );
    world.add_color(color_name, lighting);
}

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
    tuple_added_to_tuple_equals_tuple(world, tuple1, tuple2, red, green, blue, 1.0);
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
fn color_equals_shape_material_color(world: &mut RayTracerWorld, color: String, object: String) {
    let color = world.get_color(&color);
    let object = world.get_element(&object);

    let shape = match object {
        ElementType::Shape(shape) => shape,
        ElementType::SceneWorldObjectReference((parent_name, uuid)) => {
            let scene_world = world.get_world(parent_name);
            let shape = scene_world
                .elements
                .iter()
                .find(|shape| shape.id() == uuid)
                .expect("Sphere not found");
            shape
        }
        _ => panic!("Not implemented"),
    };
    assert_eq!(color, &shape.material().color);
}
