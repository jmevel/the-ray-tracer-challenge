use std::panic;

use crate::RayTracerWorld;
use crate::steps::ray_tracer_world::ElementType;
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
    match world.get_element(&tuple) {
        ElementType::Point(point) => assert_eq!(point.x(), x),
        ElementType::Color(color) => assert_eq!(color.x(), x),
        ElementType::Vector(vector) => assert_eq!(vector.x(), x),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word}.y = {float}")]
#[then(expr = "{word}.green = {float}")]
fn y_equal(world: &mut RayTracerWorld, tuple: String, y: f32) {
    match world.get_element(&tuple) {
        ElementType::Point(point) => assert_eq!(point.y(), y),
        ElementType::Color(color) => assert_eq!(color.y(), y),
        ElementType::Vector(vector) => assert_eq!(vector.y(), y),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word}.z = {float}")]
#[then(expr = "{word}.blue = {float}")]
fn z_equal(world: &mut RayTracerWorld, tuple: String, z: f32) {
    match world.get_element(&tuple) {
        ElementType::Point(point) => assert_eq!(point.z(), z),
        ElementType::Color(color) => assert_eq!(color.z(), z),
        ElementType::Vector(vector) => assert_eq!(vector.z(), z),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word}.w = {float}")]
fn w_equal(world: &mut RayTracerWorld, tuple: String, w: f32) {
    match world.get_element(&tuple) {
        ElementType::Point(point) => assert_eq!(point.w(), w),
        ElementType::Color(color) => assert_eq!(color.w(), w),
        ElementType::Vector(vector) => assert_eq!(vector.w(), w),
        _ => panic!("Not implemented"),
    }
}

// x = tuple(1.0, 2.0, 3.0, 4.0)
#[then(
    regex = r"^([a-zA-Z0-9]*) = tuple\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn tuple_equals_tuple(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    match world.get_element(&tuple) {
        ElementType::Point(point) => assert_eq!(point, &Point::new(x, y, z, w)),
        ElementType::Color(color) => assert_eq!(color, &Color::new(x, y, z, w)),
        ElementType::Vector(vector) => assert_eq!(vector, &Vector::new(x, y, z, w)),
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
    match (world.get_element(&tuple1), world.get_element(&tuple2)) {
        (ElementType::Color(color1), ElementType::Color(color2)) => {
            let expected = Color::new(x, y, z, w);
            assert_eq!(color1 + color2, expected);
        }
        (ElementType::Point(point), ElementType::Vector(vector)) => {
            let expected = Point::new(x, y, z, w);
            assert_eq!(point + vector, expected);
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
    match (world.get_element(&tuple1), world.get_element(&tuple2)) {
        (ElementType::Point(point1), ElementType::Point(point2)) => {
            assert_eq!(point1 - point2, expected);
        }
        (ElementType::Color(color1), ElementType::Color(color2)) => {
            assert_eq!(color1 - color2, expected);
        }
        (ElementType::Vector(vector1), ElementType::Vector(vector2)) => {
            assert_eq!(vector1 - vector2, expected);
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
    match world.get_element(&tuple) {
        ElementType::Point(point) => {
            let expected = Point::new(x, y, z, w);
            assert_eq!(-point, expected);
        }
        ElementType::Color(color) => {
            let expected = Color::new(x, y, z, w);
            assert_eq!(-color, expected);
        }
        ElementType::Vector(vector) => {
            let expected = Vector::new(x, y, z, w);
            assert_eq!(-vector, expected);
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
    match world.get_element(&tuple) {
        ElementType::Point(point) => {
            let expected = Point::new(x, y, z, w);
            assert_eq!(point * scalar, expected);
        }
        ElementType::Color(color) => {
            let expected = Color::new(x, y, z, w);
            assert_eq!(color * scalar, expected);
        }
        ElementType::Vector(vector) => {
            let expected = Vector::new(x, y, z, w);
            assert_eq!(vector * scalar, expected);
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
    match world.get_element(&tuple) {
        ElementType::Point(point) => {
            let expected = Point::new(x, y, z, w);
            assert_eq!(point / fraction, expected);
        }
        ElementType::Color(color) => {
            let expected = Color::new(x, y, z, w);
            assert_eq!(color / fraction, expected);
        }
        ElementType::Vector(vector) => {
            let expected = Vector::new(x, y, z, w);
            assert_eq!(vector / fraction, expected);
        }
        _ => panic!("Not implemented"),
    }
}
