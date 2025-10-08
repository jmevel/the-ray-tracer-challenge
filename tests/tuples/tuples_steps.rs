use cucumber::{given, then, when, World};
use std::collections::HashMap;
use the_ray_tracer_challenge::color::Color;
use the_ray_tracer_challenge::float::float_equals;
use the_ray_tracer_challenge::point::Point;
use the_ray_tracer_challenge::vector::Vector;
use the_ray_tracer_challenge::ExtendedTuple;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    points: HashMap<String, Point>,
    vectors: HashMap<String, Vector>,
    colors: HashMap<String, Color>,
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn tuple_is(world: &mut TupleWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    match w {
        0f32 => {
            world.vectors.insert(tuple, Vector::new(x, y, z, Some(w)));
        }
        1f32 => {
            world.points.insert(tuple, Point::new(x, y, z, Some(w)));
        }
        _ => panic!("w out of bounds"),
    };
}

#[given(expr = "{word} ← point\\({float}, {float}, {float})")]
#[given(expr = "{word} ← color\\({float}, {float}, {float})")]
fn tuple_is_point(world: &mut TupleWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.points.insert(tuple, Point::new(x, y, z, None));
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
fn tuple_is_vector(world: &mut TupleWorld, tuple: String, x: f32, y: f32, z: f32) {
    world.vectors.insert(tuple, Vector::new(x, y, z, None));
}

#[when(expr = "{word} ← normalize\\({word})")]
fn tuple_is_normalization(world: &mut TupleWorld, vector1: String, vector2: String) {
    let tuple1 = world.get_vector(vector1);
    world.vectors.insert(vector2, tuple1.normalize());
}

#[then(expr = "{word}.x = {float}")]
#[then(expr = "{word}.red = {float}")]
fn x_equal(world: &mut TupleWorld, tuple: String, x: f32) {
    assert_eq!(world.get_tuple(tuple).x(), &x);
}

#[then(expr = "{word}.y = {float}")]
#[then(expr = "{word}.green = {float}")]
fn y_equal(world: &mut TupleWorld, tuple: String, y: f32) {
    assert_eq!(world.get_tuple(tuple).y(), &y);
}

#[then(expr = "{word}.z = {float}")]
#[then(expr = "{word}.blue = {float}")]
fn z_equal(world: &mut TupleWorld, tuple: String, z: f32) {
    assert_eq!(world.get_tuple(tuple).z(), &z);
}

#[then(expr = "{word}.w = {float}")]
fn w_equal(world: &mut TupleWorld, tuple: String, w: f32) {
    assert_eq!(world.get_tuple(tuple).w(), &w);
}

#[then(expr = "{word} is a point")]
fn is_a_point(world: &mut TupleWorld, point: String) {
    world.get_point(point);
}

#[then(expr = "{word} is a vector")]
fn is_a_vector(world: &mut TupleWorld, tuple: String) {
    world.get_vector(tuple);
}

#[then(expr = "{word} is not a point")]
fn is_not_a_point(world: &mut TupleWorld, tuple: String) {
    let result = std::panic::catch_unwind(|| world.get_point(tuple));
    assert!(result.is_err());
}

#[then(expr = "{word} is not a vector")]
fn is_not_a_vector(world: &mut TupleWorld, tuple: String) {
    let result = std::panic::catch_unwind(|| world.get_vector(tuple));
    assert!(result.is_err());
}

#[then(
    regex = r"^([a-zA-Z0-9]*) = tuple\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn equals_tuple(world: &mut TupleWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(*world.get_tuple(tuple), ExtendedTuple::new(x, y, z, w));
}

#[then(regex = r"^([a-zA-Z0-9]*) = ([a-zA-Z0-9]*)$")]
fn tuple_equals_tuple(world: &mut TupleWorld, tuple1: String, tuple2: String) {
    assert_eq!(world.get_tuple(tuple1), world.get_tuple(tuple2));
}

#[then(expr = "{word} != {word}")]
fn tuple_does_not_equal_tuple(world: &mut TupleWorld, tuple1: String, tuple2: String) {
    assert!(!(world.get_point(tuple1) == world.get_point(tuple2)));
}

#[then(expr = "{word} + {word} = tuple\\({float}, {float}, {float}, {float})")]
fn tuple_added_to_tuple_equals_tuple(
    world: &mut TupleWorld,
    tuple1: String,
    tuple2: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let expected = match w {
        0f32 => &Vector::new(x, y, z, None) as &dyn ExtendedTuple,
        1f32 => &Point::new(x, y, z, None) as &dyn ExtendedTuple,
        _ => panic!("w out of bounds"),
    };
    let tuple1 = world.get_tuple(tuple1);
    let tuple2 = world.get_tuple(tuple2);

    let result = tuple1 + tuple2;
    assert_eq!(result, expected);
}

#[then(expr = "{word} + {word} = color\\({float}, {float}, {float})")]
fn color_added_to_color_equals_color(
    world: &mut TupleWorld,
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
    world: &mut TupleWorld,
    point1: String,
    point2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let expected = Vector::new(x, y, z, None);
    let point1 = world.get_point(point1);
    let point2 = world.get_point(point2);

    let result = point1 - point2;
    assert_eq!(result, expected);
}

#[then(expr = "{word} - {word} = point\\({float}, {float}, {float})")]
fn point_subtracted_to_vector_equals_point(
    world: &mut TupleWorld,
    point: String,
    vector: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let expected = Point::new(x, y, z, None);
    let point = world.get_point(point);
    let vector = world.get_vector(vector);

    let result = point - vector;
    assert_eq!(result, expected);
}

#[then(expr = "{word} - {word} = color\\({float}, {float}, {float})")]
fn color_subtracted_to_color_equals_color(
    world: &mut TupleWorld,
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
    world: &mut TupleWorld,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let expected = ExtendedTuple::new(x, y, z, Some(w));
    let tuple = world.get_vector(tuple);

    let result = -tuple;
    assert_eq!(result, expected);
}

#[then(expr = "{word} * {float} = tuple\\({float}, {float}, {float}, {float})")]
fn multiplied_tuple_by_scalar_equals_tuple(
    world: &mut TupleWorld,
    tuple: String,
    scalar: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let expected = Vector::new(x, y, z, Some(w));
    let tuple = world.get_vector(tuple);

    let result = tuple * scalar;
    assert_eq!(result, expected);
}

#[then(expr = "{word} * {word} = color\\({float}, {float}, {float})")]
fn multiplied_color_by_color_equals_color(
    world: &mut TupleWorld,
    color1: String,
    scalar_or_color: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    let expected = Color::new(red, green, blue);
    let color1 = world.get_color(color1);

    let result = match scalar_or_color.parse::<f32>() {
        Ok(scalar) => color1 * scalar,
        Err(_) => {
            let color2 = world.get_color(scalar_or_color);
            color1 * color2
        }
    };

    assert!(float_equals(&result.red(), &expected.red()));
    assert!(float_equals(&result.green(), &expected.green()));
    assert!(float_equals(&result.blue(), &expected.blue()));
}

#[then(expr = "{word} \\/ {int} = tuple\\({float}, {float}, {float}, {float})")]
fn divided_tuple_by_fraction_equals_tuple(
    world: &mut TupleWorld,
    tuple: String,
    fraction: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let expected = Vector::new(x, y, z, Some(w));

    let tuple = world.get_vector(tuple);

    let result = tuple / fraction;
    assert_eq!(result, expected);
}

#[then(expr = "magnitude\\({word}) = {float}")]
fn magnitude_equals_float(world: &mut TupleWorld, vector: String, expected: f32) {
    let vector = world.get_vector(vector);

    assert!(float_equals(&vector.magnitude(), &expected));
}

#[then(expr = "magnitude\\({word}) = √{float}")]
fn magnitude_equals_squareroot_float(world: &mut TupleWorld, vector: String, expected: f32) {
    let expected = expected.sqrt();
    let vector = world.get_vector(vector);

    assert_eq!(vector.magnitude(), expected);
}

#[then(expr = "normalize\\({word}) = vector\\({float}, {float}, {float})")]
fn normalize_equals_vector(world: &mut TupleWorld, vector: String, x: f32, y: f32, z: f32) {
    let expected = Vector::new(x, y, z, None);
    let vector = world.get_vector(vector);

    assert_eq!(vector.normalize(), expected);
}

#[then(expr = "normalize\\({word}) = approximately vector\\({float}, {float}, {float})")]
fn normalize_equals_approximately_vector(
    world: &mut TupleWorld,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
) {
    normalize_equals_vector(world, tuple, x, y, z);
}

#[then(expr = "dot\\({word}, {word}) = {float}")]
fn dot_two_vectors_equals(world: &mut TupleWorld, vector1: String, vector2: String, expected: f32) {
    let tuple1 = world.get_vector(vector1);
    let tuple2 = world.get_vector(vector2);

    assert_eq!(tuple1.dot_product(tuple2), expected);
}

#[then(expr = "cross\\({word}, {word}) = vector\\({float}, {float}, {float})")]
fn cross_two_vectors_equals_vector(
    world: &mut TupleWorld,
    vector1: String,
    vector2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let tuple1 = world.get_vector(vector1);
    let tuple2 = world.get_vector(vector2);

    let expected = Vector::new(x, y, z, None);

    assert_eq!(tuple1.cross_product(tuple2), expected);
}

impl TupleWorld {
    fn get_tuple(&self, tuple: String) -> &dyn ExtendedTuple {
        match self.points.get(&tuple) {
            Some(point) => point as &dyn ExtendedTuple,
            None => match self.vectors.get(&tuple) {
                Some(vector) => vector,
                None => panic!("{}", format!("{tuple} not found")),
            },
        }
    }
    fn get_point(&self, point: String) -> &Point {
        self.points
            .get(&point)
            .expect(format!("{point} does not exist").as_str())
    }
    fn get_vector(&self, vector: String) -> &Vector {
        self.vectors
            .get(&vector)
            .expect(format!("{vector} does not exist").as_str())
    }
    fn get_color(&self, color: String) -> &Color {
        self.colors
            .get(&color)
            .expect(format!("{color} does not exist").as_str())
    }
}
