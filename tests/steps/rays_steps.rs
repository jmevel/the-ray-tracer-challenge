use cucumber::{then, when};
use the_ray_tracer_challenge::Ray;

use crate::steps::ray_tracer_world::RayTracerWorld;

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
