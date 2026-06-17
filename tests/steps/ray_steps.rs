use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Point, Ray, Vector};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(
    expr = "{word} ← ray\\(point\\({float}, {float}, {float}), vector\\({float}, {float}, {float}))"
)]
fn ray_is_point_and_vector(
    world: &mut RayTracerWorld,
    ray_name: String,
    origin_x: f32,
    origin_y: f32,
    origin_z: f32,
    direction_x: f32,
    direction_y: f32,
    direction_z: f32,
) {
    let origin = Point::new_point(origin_x, origin_y, origin_z);
    let direction = Vector::new_vector(direction_x, direction_y, direction_z);
    let ray = Ray::new(origin, direction);
    world.add_ray(ray_name, ray);
}

#[when(expr = "{word} ← ray\\({word}, {word})")]
fn ray_is_origin_and_direction(
    world: &mut RayTracerWorld,
    ray_name: String,
    origin: String,
    direction: String,
) {
    let origin = world.get_point(&origin);
    let direction = world.get_vector(&direction);
    let ray = Ray::new(origin.clone(), direction.clone());
    world.add_ray(ray_name, ray);
}

#[when(expr = "{word} ← transform\\({word}, {word})")]
fn ray_is_ray_transformed_by_matrix(
    world: &mut RayTracerWorld,
    new_ray_name: String,
    initial_ray: String,
    transformation: String,
) {
    let initial_ray = world.get_ray(&initial_ray);
    let transformation = world.get_matrix4x4(&transformation);
    let new_ray = initial_ray.transform(transformation);
    world.add_ray(new_ray_name, new_ray);
}

#[then(expr = "{word}.origin = {word}")]
fn ray_origin_equals_origin(world: &mut RayTracerWorld, ray: String, origin: String) {
    let ray = world.get_ray(&ray);
    let expected_origin = world.get_point(&origin);
    assert_eq!(ray.origin(), expected_origin);
}

#[then(expr = "{word}.direction = {word}")]
fn ray_direction_equals_direction(world: &mut RayTracerWorld, ray: String, direction: String) {
    let ray = world.get_ray(&ray);
    let expected_direction = world.get_vector(&direction);
    assert_eq!(ray.direction(), expected_direction);
}

#[then(expr = "position\\({word}, {float}) = point\\({float}, {float}, {float})")]
fn position_of_ray_equals_point(
    world: &mut RayTracerWorld,
    ray: String,
    position: f32,
    x: f32,
    y: f32,
    z: f32,
) {
    let ray = world.get_ray(&ray);
    let expected_position = Point::new_point(x, y, z);
    assert_eq!(ray.position(&position), expected_position);
}

#[then(expr = "{word}.origin = point\\({float}, {float}, {float})")]
fn ray_origin_equals_point(world: &mut RayTracerWorld, ray: String, x: f32, y: f32, z: f32) {
    let expected = Point::new_point(x, y, z);
    let ray = world.get_ray(&ray);
    assert_eq!(ray.origin(), &expected);
}

#[then(expr = "{word}.direction = vector\\({float}, {float}, {float})")]
fn ray_direction_equals_vector(world: &mut RayTracerWorld, ray: String, x: f32, y: f32, z: f32) {
    let expected = Vector::new_vector(x, y, z);
    let ray = world.get_ray(&ray);
    assert_eq!(ray.direction(), &expected);
}
