use std::panic;

use cucumber::then;
use the_ray_tracer_challenge::{Color, Tuple, float::float_equals};

use crate::steps::{
    ray_tracer_world::RayTracerWorld, tuple_steps::tuple_added_to_tuple_equals_tuple,
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
