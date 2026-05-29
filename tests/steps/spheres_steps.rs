use cucumber::{given, when};
use the_ray_tracer_challenge::Sphere;

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← sphere\\()")]
fn sphere_is(world: &mut RayTracerWorld, sphere_name: String) {
    let sphere = Sphere::new();
    world.add_sphere(sphere_name, sphere);
}

#[when(expr = "{word} ← intersect\\({word}, {word})")]
fn intersection_is_intersect_of_sphere_and_ray(
    world: &mut RayTracerWorld,
    intersect_name: String,
    sphere_name: String,
    ray_name: String,
) {
    let sphere = world.get_sphere(&sphere_name);
    let ray = world.get_ray(&ray_name);
    let intersections = sphere.intersect(ray);
    world.add_intersections_collection(intersect_name, intersections);
}
