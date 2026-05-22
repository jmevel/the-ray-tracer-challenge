use cucumber::{given, then, when};
use the_ray_tracer_challenge::Sphere;

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← sphere\\()")]
fn sphere_is(world: &mut RayTracerWorld, sphere_name: String) {
    let sphere = Sphere::new();
    world.add_sphere(sphere_name, sphere);
}

#[when(expr = "{word} ← intersect\\({word}, {word})")]
fn intersect_is_intersect_of_sphere_and_ray(
    world: &mut RayTracerWorld,
    intersect_name: String,
    sphere_name: String,
    ray_name: String,
) {
    let sphere = world.get_sphere(&sphere_name);
    let ray = world.get_ray(&ray_name);
    let intersect = sphere.intersect(ray);
    world.add_intersect(intersect_name, intersect);
}

#[then(expr = "{word}.count = {int}")]
fn intersect_count_equals_count(world: &mut RayTracerWorld, intersect_name: String, count: usize) {
    if count == 0 {
        let intersect = world.get_intersect(&intersect_name);
        assert!(intersect.is_none());
    } else {
        let intersect = world
            .get_intersect(&intersect_name)
            .clone()
            .expect("No intersect found");

        assert_eq!(&intersect.iter().count(), &count);
    }
}

#[then(expr = "{word}[{int}] = {float}")]
fn index_of_intersect_equals_value(
    world: &mut RayTracerWorld,
    intersect_name: String,
    index: usize,
    value: f32,
) {
    let intersect = world
        .get_intersect(&intersect_name)
        .clone()
        .expect("No intersect found");

    assert_eq!(intersect[index], value);
}
