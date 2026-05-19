use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Ray, Tuple};

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
    let origin = Tuple::new_point(origin_x, origin_y, origin_z);
    let direction = Tuple::new_vector(direction_x, direction_y, direction_z);
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
    let origin = world.get_tuple(&origin);
    let direction = world.get_tuple(&direction);
    let ray = Ray::new(origin.clone(), direction.clone());
    world.add_ray(ray_name, ray);
}

#[then(expr = "{word}.origin = {word}")]
fn ray_origin_equals_origin(world: &mut RayTracerWorld, ray: String, origin: String) {
    let ray = world.get_ray(&ray);
    let expected_origin = world.get_tuple(&origin);
    assert_eq!(ray.origin(), expected_origin);
}

#[then(expr = "{word}.direction = {word}")]
fn ray_direction_equals_direction(world: &mut RayTracerWorld, ray: String, direction: String) {
    let ray = world.get_ray(&ray);
    let expected_direction = world.get_tuple(&direction);
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
    let expected_position = Tuple::new_point(x, y, z);
    assert_eq!(ray.position(position), expected_position);
}
