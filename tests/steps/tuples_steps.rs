use std::panic;

use crate::RayTracerWorld;
use crate::steps::ray_tracer_world::Type;
use cucumber::{given, then, when};
use the_ray_tracer_challenge::float::float_equals;
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

#[then(expr = "{word} is a point")]
fn is_a_point(world: &mut RayTracerWorld, tuple: String) {
    _ = world.get_point(&tuple);
}

#[then(expr = "{word} is a vector")]
fn is_a_vector(world: &mut RayTracerWorld, tuple: String) {
    _ = world.get_vector(&tuple);
}

#[then(expr = "{word} is not a point")]
fn is_not_a_point(world: &mut RayTracerWorld, tuple: String) {
    let result = panic::catch_unwind(|| world.get_point(&tuple));
    assert!(result.is_err());
}

#[then(expr = "{word} is not a vector")]
fn is_not_a_vector(world: &mut RayTracerWorld, tuple: String) {
    let result = panic::catch_unwind(|| world.get_vector(&tuple));
    assert!(result.is_err());
}

// x = tuple(1.0, 2.0, 3.0, 4.0)
#[then(
    regex = r"^([a-zA-Z0-9]*) = tuple\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn equals_tuple(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    match world.get_element_type(&tuple) {
        Type::Point => assert_eq!(world.get_point(&tuple), &Point::new(x, y, z, w)),
        Type::Color => assert_eq!(world.get_color(&tuple), &Color::new(x, y, z, w)),
        Type::Vector => assert_eq!(world.get_vector(&tuple), &Vector::new(x, y, z, w)),
        _ => panic!("Not implemented"),
    }
}

#[then(expr = "{word} + {word} = tuple\\({float}, {float}, {float}, {float})")]
fn tuple_added_to_tuple_equals_tuple(
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

#[then(expr = "{word} + {word} = color\\({float}, {float}, {float})")]
fn color_added_to_color_equals_color(
    world: &mut RayTracerWorld,
    tuple1: String,
    tuple2: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    tuple_added_to_tuple_equals_tuple(world, tuple1, tuple2, red, green, blue, 2f32);
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

#[then(
    regex = r#"^([a-zA-Z0-9_]+) = point\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"#
)]
fn point_equals_point(world: &mut RayTracerWorld, point: String, x: f32, y: f32, z: f32) {
    let expected = Point::new_point(x, y, z);
    let actual = world.get_point(&point);
    assert_eq!(actual, &expected);
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

#[then(expr = "magnitude\\({word}) = {float}")]
fn magnitude_equals_float(world: &mut RayTracerWorld, tuple: String, expected: f32) {
    let tuple = world.get_vector(&tuple);

    assert!(float_equals(&tuple.magnitude(), &expected));
}

#[then(expr = "magnitude\\({word}) = √{float}")]
fn magnitude_equals_squareroot_float(world: &mut RayTracerWorld, tuple: String, expected: f32) {
    let expected = expected.sqrt();
    let tuple = world.get_vector(&tuple);

    assert_eq!(tuple.magnitude(), expected);
}

#[then(expr = "normalize\\({word}) = vector\\({float}, {float}, {float})")]
fn normalize_equals_vector(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    let expected = Vector::new_vector(x, y, z);
    let tuple = world.get_vector(&tuple);

    assert_eq!(tuple.normalize(), expected);
}

#[then(expr = "normalize\\({word}) = approximately vector\\({float}, {float}, {float})")]
fn normalize_equals_approximately_vector(
    world: &mut RayTracerWorld,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
) {
    normalize_equals_vector(world, tuple, x, y, z);
}

#[then(expr = "dot\\({word}, {word}) = {float}")]
fn dot_two_vectors_equals(
    world: &mut RayTracerWorld,
    vector1: String,
    vector2: String,
    expected: f32,
) {
    let vector1 = world.get_vector(&vector1);
    let vector2 = world.get_vector(&vector2);

    assert_eq!(vector1.dot_product(vector2), expected);
}

#[then(expr = "cross\\({word}, {word}) = vector\\({float}, {float}, {float})")]
fn cross_two_vectors_equals_vector(
    world: &mut RayTracerWorld,
    vector1: String,
    vector2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let vector1 = world.get_vector(&vector1);
    let vector2 = world.get_vector(&vector2);
    let expected = Vector::new_vector(x, y, z);

    assert_eq!(vector1.cross_product(vector2), expected);
}
