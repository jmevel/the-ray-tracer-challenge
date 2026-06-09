use std::panic;

use crate::RayTracerWorld;
use crate::steps::ray_tracer_world::Type;
use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Color, Point, Tuple, Vector};

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn tuple_is(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    match w {
        1f32 => world.add_point(tuple, Point::new(x, y, z, w)),
        _ => world.add_vector(tuple, Vector::new(x, y, z, w)),
    }
}

#[given(expr = "{word} ← point\\({float}, {float}, {float})")]
fn tuple_is_point(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.add_point(tuple, Point::new_point(x, y, z));
}

#[given(expr = "{word} ← color\\({float}, {float}, {float})")]
fn tuple_is_color(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.add_color(tuple, Color::new_color(x, y, z));
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
pub fn tuple_is_vector(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.add_vector(tuple, Vector::new_vector(x, y, z));
}

#[given(expr = "{word} ← vector\\(√{float}\\/{float}, √{float}\\/{float}, {float})")]
pub fn tuple_is_vector2(
    world: &mut RayTracerWorld,
    tuple: String,
    x_numerator: f32,
    x_denominator: f32,
    y_numerator: f32,
    y_denominator: f32,
    z: f32,
) {
    let x = f32::sqrt(x_numerator) / x_denominator;
    let y = f32::sqrt(y_numerator) / y_denominator;
    world.add_vector(tuple, Vector::new_vector(x, y, z));
}

#[when(expr = "{word} ← normalize\\({word})")]
fn tuple_is_normalization(world: &mut RayTracerWorld, tuple2: String, tuple1: String) {
    let tuple1 = world.get_vector(&tuple1);
    world.add_vector(tuple2, tuple1.normalize());
}

#[then(expr = "{word}.x = {float}")]
#[then(expr = "{word}.red = {float}")]
fn x_equal(world: &mut RayTracerWorld, tuple: String, x: f32) {
    match world.get_element_type(&tuple) {
        Type::Point => assert_eq!(world.get_point(&tuple).x(), &x),
        Type::Color => assert_eq!(world.get_color(&tuple).x(), &x),
        Type::Vector => assert_eq!(world.get_vector(&tuple).x(), &x),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word}.y = {float}")]
#[then(expr = "{word}.green = {float}")]
fn y_equal(world: &mut RayTracerWorld, tuple: String, y: f32) {
    match world.get_element_type(&tuple) {
        Type::Point => assert_eq!(world.get_point(&tuple).y(), &y),
        Type::Color => assert_eq!(world.get_color(&tuple).y(), &y),
        Type::Vector => assert_eq!(world.get_vector(&tuple).y(), &y),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word}.z = {float}")]
#[then(expr = "{word}.blue = {float}")]
fn z_equal(world: &mut RayTracerWorld, tuple: String, z: f32) {
    match world.get_element_type(&tuple) {
        Type::Point => assert_eq!(world.get_point(&tuple).z(), &z),
        Type::Color => assert_eq!(world.get_color(&tuple).z(), &z),
        Type::Vector => assert_eq!(world.get_vector(&tuple).z(), &z),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word}.w = {float}")]
fn w_equal(world: &mut RayTracerWorld, tuple: String, w: f32) {
    match world.get_element_type(&tuple) {
        Type::Point => assert_eq!(world.get_point(&tuple).w(), &w),
        Type::Color => assert_eq!(world.get_color(&tuple).w(), &w),
        Type::Vector => assert_eq!(world.get_vector(&tuple).w(), &w),
        _ => panic!("Not implemented"),
    }
}

// x = tuple(1.0, 2.0, 3.0, 4.0)
#[then(
    regex = r"^([a-zA-Z0-9]*) = tuple\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn tuple_equals_tuple(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    match world.get_element_type(&tuple) {
        Type::Point => assert_eq!(world.get_point(&tuple), &Point::new(x, y, z, w)),
        Type::Color => assert_eq!(world.get_color(&tuple), &Color::new(x, y, z, w)),
        Type::Vector => assert_eq!(world.get_vector(&tuple), &Vector::new(x, y, z, w)),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word} + {word} = tuple\\({float}, {float}, {float}, {float})")]
pub fn tuple_added_to_tuple_equals_tuple(
    world: &mut RayTracerWorld,
    tuple1: String,
    tuple2: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    match (
        world.get_element_type(&tuple1),
        world.get_element_type(&tuple2),
    ) {
        (Type::Color, Type::Color) => {
            let expected = Color::new(x, y, z, w);
            let tuple1 = world.get_color(&tuple1);
            let tuple2 = world.get_color(&tuple2);
            let result = tuple1 + tuple2;
            assert_eq!(result, expected);
        }
        (Type::Point, Type::Vector) => {
            let expected = Point::new(x, y, z, w);
            let point = world.get_point(&tuple1);
            let vector = world.get_vector(&tuple2);
            let result = point + vector;
            assert_eq!(result, expected);
        }

        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word} - {word} = vector\\({float}, {float}, {float})")]
fn tuple_subtracted_to_tuple_equals_vector(
    world: &mut RayTracerWorld,
    tuple1: String,
    tuple2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let expected = Vector::new_vector(x, y, z);
    match (
        world.get_element_type(&tuple1),
        world.get_element_type(&tuple2),
    ) {
        (Type::Point, Type::Point) => {
            let point1 = world.get_point(&tuple1);
            let point2 = world.get_point(&tuple2);
            let result = point1 - point2;
            assert_eq!(result, expected);
        }
        (Type::Color, Type::Color) => {
            let color1 = world.get_color(&tuple1);
            let color2 = world.get_color(&tuple2);
            let result = color1 - color2;
            assert_eq!(result, expected);
        }
        (Type::Vector, Type::Vector) => {
            let vector1 = world.get_vector(&tuple1);
            let vector2 = world.get_vector(&tuple2);
            let result = vector1 - vector2;
            assert_eq!(result, expected);
        }
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word} - {word} = point\\({float}, {float}, {float})")]
fn point_subtracted_to_vector_equals_point(
    world: &mut RayTracerWorld,
    point: String,
    vector: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let expected = Point::new_point(x, y, z);
    let point = world.get_point(&point);
    let vector = world.get_vector(&vector);
    let result = point - vector;

    assert_eq!(result, expected);
}

#[then(expr = "{word} - {word} = color\\({float}, {float}, {float})")]
fn color_subtracted_to_color_equals_color(
    world: &mut RayTracerWorld,
    tuple1: String,
    tuple2: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    tuple_subtracted_to_tuple_equals_vector(world, tuple1, tuple2, red, green, blue);
}

#[then(expr = "-{word} = tuple\\({float}, {float}, {float}, {float})")]
fn negated_tuple_equals_tuple(
    world: &mut RayTracerWorld,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    match world.get_element_type(&tuple) {
        Type::Point => {
            let expected = Point::new(x, y, z, w);
            let tuple = world.get_point(&tuple);
            let result = -tuple;
            assert_eq!(result, expected);
        }
        Type::Color => {
            let expected = Color::new(x, y, z, w);
            let tuple = world.get_color(&tuple);
            let result = -tuple;
            assert_eq!(result, expected);
        }
        Type::Vector => {
            let expected = Vector::new(x, y, z, w);
            let tuple = world.get_vector(&tuple);
            let result = -tuple;
            assert_eq!(result, expected);
        }
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word} * {float} = tuple\\({float}, {float}, {float}, {float})")]
fn multiplied_tuple_by_scalar_equals_tuple(
    world: &mut RayTracerWorld,
    tuple: String,
    scalar: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    match world.get_element_type(&tuple) {
        Type::Point => {
            let expected = Point::new(x, y, z, w);
            let tuple = world.get_point(&tuple);
            let result = tuple * scalar;
            assert_eq!(result, expected);
        }
        Type::Color => {
            let expected = Color::new(x, y, z, w);
            let tuple = world.get_color(&tuple);
            let result = tuple * scalar;
            assert_eq!(result, expected);
        }
        Type::Vector => {
            let expected = Vector::new(x, y, z, w);
            let tuple = world.get_vector(&tuple);
            let result = tuple * scalar;
            assert_eq!(result, expected);
        }
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word} \\/ {int} = tuple\\({float}, {float}, {float}, {float})")]
fn divided_tuple_by_fraction_equals_tuple(
    world: &mut RayTracerWorld,
    tuple: String,
    fraction: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    match world.get_element_type(&tuple) {
        Type::Point => {
            let expected = Point::new(x, y, z, w);
            let tuple = world.get_point(&tuple);
            let result = tuple / fraction;
            assert_eq!(result, expected);
        }
        Type::Color => {
            let expected = Color::new(x, y, z, w);
            let tuple = world.get_color(&tuple);
            let result = tuple / fraction;
            assert_eq!(result, expected);
        }
        Type::Vector => {
            let expected = Vector::new(x, y, z, w);
            let tuple = world.get_vector(&tuple);
            let result = tuple / fraction;
            assert_eq!(result, expected);
        }
        _ => panic!("Not implemented"),
    }
}
