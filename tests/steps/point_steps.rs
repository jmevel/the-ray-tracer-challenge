use cucumber::then;
use the_ray_tracer_challenge::Point;

use crate::steps::ray_tracer_world::RayTracerWorld;

#[then(
    regex = r"^(\w+) = point\(([+-]?\d+(?:\.\d+)?), ([+-]?\d+(?:\.\d+)?), ([+-]?\d+(?:\.\d+)?)\)$"
)]
fn point_equals_point(world: &mut RayTracerWorld, point: String, x: f32, y: f32, z: f32) {
    let expected = Point::new_point(x, y, z);
    let actual = world.get_point(&point);
    assert_eq!(actual, &expected);
}
