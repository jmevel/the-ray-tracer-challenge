use crate::RayTracerWorld;
use cucumber::{given, then, when};
use the_ray_tracer_challenge::Tuple;
use the_ray_tracer_challenge::float::float_equals;

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn tuple_is(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    world.add_tuple(tuple, Tuple::new(x, y, z, w));
}

#[given(expr = "{word} ← point\\({float}, {float}, {float})")]
fn tuple_is_point(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.add_tuple(tuple, Tuple::new_point(x, y, z));
}

#[given(expr = "{word} ← color\\({float}, {float}, {float})")]
fn tuple_is_color(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.add_tuple(tuple, Tuple::new_color(x, y, z));
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
pub fn tuple_is_vector(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.add_tuple(tuple, Tuple::new_vector(x, y, z));
}

#[when(expr = "{word} ← normalize\\({word})")]
fn tuple_is_normalization(world: &mut RayTracerWorld, tuple2: String, tuple1: String) {
    let tuple1 = world.get_tuple(&tuple1);
    world.add_tuple(tuple2, tuple1.normalize());
}

#[then(expr = "{word}.x = {float}")]
#[then(expr = "{word}.red = {float}")]
fn x_equal(world: &mut RayTracerWorld, tuple: String, x: f32) {
    assert_eq!(world.get_tuple(&tuple).x(), &x);
}

#[then(expr = "{word}.y = {float}")]
#[then(expr = "{word}.green = {float}")]
fn y_equal(world: &mut RayTracerWorld, tuple: String, y: f32) {
    assert_eq!(world.get_tuple(&tuple).y(), &y);
}

#[then(expr = "{word}.z = {float}")]
#[then(expr = "{word}.blue = {float}")]
fn z_equal(world: &mut RayTracerWorld, tuple: String, z: f32) {
    assert_eq!(world.get_tuple(&tuple).z(), &z);
}

#[then(expr = "{word}.w = {float}")]
fn w_equal(world: &mut RayTracerWorld, tuple: String, w: f32) {
    assert_eq!(world.get_tuple(&tuple).w(), &w);
}

#[then(expr = "{word} is a point")]
fn is_a_point(world: &mut RayTracerWorld, tuple: String) {
    assert!(world.get_tuple(&tuple).is_point());
}

#[then(expr = "{word} is a vector")]
fn is_a_vector(world: &mut RayTracerWorld, tuple: String) {
    assert!(world.get_tuple(&tuple).is_vector());
}

#[then(expr = "{word} is not a point")]
fn is_not_a_point(world: &mut RayTracerWorld, tuple: String) {
    assert!(!world.get_tuple(&tuple).is_point());
}

#[then(expr = "{word} is not a vector")]
fn is_not_a_vector(world: &mut RayTracerWorld, tuple: String) {
    assert!(!world.get_tuple(&tuple).is_vector());
}

#[then(
    regex = r"^([a-zA-Z0-9]*) = tuple\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn equals_tuple(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(*world.get_tuple(&tuple), Tuple::new(x, y, z, w));
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
    let expected = Tuple::new(x, y, z, w);
    let tuple1 = world.get_tuple(&tuple1);
    let tuple2 = world.get_tuple(&tuple2);
    let result = tuple1 + tuple2;

    assert_eq!(result, expected);
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
fn point_subtracted_to_point_equals_vector(
    world: &mut RayTracerWorld,
    point1: String,
    point2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let expected = Tuple::new_vector(x, y, z);
    let point1 = world.get_tuple(&point1);
    let point2 = world.get_tuple(&point2);
    let result = point1 - point2;

    assert_eq!(result, expected);
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
    let expected = Tuple::new_point(x, y, z);
    let point = world.get_tuple(&point);
    let vector = world.get_tuple(&vector);
    let result = point - vector;

    assert_eq!(result, expected);
}

#[then(expr = "{word} = point\\({float}, {float}, {float})")]
fn point_equals_point(world: &mut RayTracerWorld, point: String, x: f32, y: f32, z: f32) {
    let expected = Tuple::new_point(x, y, z);
    let actual = world.get_tuple(&point);
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
    point_subtracted_to_point_equals_vector(world, tuple1, tuple2, red, green, blue);
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
    let expected = Tuple::new(x, y, z, w);
    let tuple = world.get_tuple(&tuple);
    let result = -tuple;

    assert_eq!(result, expected);
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
    let expected = Tuple::new(x, y, z, w);
    let tuple = world.get_tuple(&tuple);
    let result = tuple * scalar;

    assert_eq!(result, expected);
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
    let expected = Tuple::new_color(red, green, blue);
    let color1 = world.get_tuple(&color1);

    let result = match scalar_or_color.parse::<f32>() {
        Ok(scalar) => color1 * scalar,
        Err(_) => {
            let color2 = world.get_tuple(&scalar_or_color);
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
    let expected = Tuple::new(x, y, z, w);
    let tuple = world.get_tuple(&tuple);
    let result = tuple / fraction;

    assert_eq!(result, expected);
}

#[then(expr = "magnitude\\({word}) = {float}")]
fn magnitude_equals_float(world: &mut RayTracerWorld, tuple: String, expected: f32) {
    let tuple = world.get_tuple(&tuple);

    assert!(float_equals(&tuple.magnitude(), &expected));
}

#[then(expr = "magnitude\\({word}) = √{float}")]
fn magnitude_equals_squareroot_float(world: &mut RayTracerWorld, tuple: String, expected: f32) {
    let expected = expected.sqrt();
    let tuple = world.get_tuple(&tuple);

    assert_eq!(tuple.magnitude(), expected);
}

#[then(expr = "normalize\\({word}) = vector\\({float}, {float}, {float})")]
fn normalize_equals_vector(world: &mut RayTracerWorld, tuple: String, x: f32, y: f32, z: f32) {
    let expected = Tuple::new_vector(x, y, z);
    let tuple = world.get_tuple(&tuple);

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
    tuple1: String,
    tuple2: String,
    expected: f32,
) {
    let tuple1 = world.get_tuple(&tuple1);
    let tuple2 = world.get_tuple(&tuple2);

    assert_eq!(tuple1.dot_product(tuple2), expected);
}

#[then(expr = "cross\\({word}, {word}) = vector\\({float}, {float}, {float})")]
fn cross_two_vectors_equals_vector(
    world: &mut RayTracerWorld,
    tuple1: String,
    tuple2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let tuple1 = world.get_tuple(&tuple1);
    let tuple2 = world.get_tuple(&tuple2);
    let expected = Tuple::new_vector(x, y, z);

    assert_eq!(tuple1.cross_product(tuple2), expected);
}
