use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Matrix, Sphere};

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
) -> Result<(), String> {
    let sphere = world.get_sphere(&sphere_name);
    let ray = world.get_ray(&ray_name);
    let intersections = sphere.intersect(ray);
    world.add_intersections_collection(intersect_name, intersections?);
    Ok(())
}

#[when(expr = "set_transform\\({word}, {word})")]
fn set_transform_s_t(world: &mut RayTracerWorld, sphere: String, transformation: String) {
    let transformation = world.get_matrix4x4(&transformation);
    world.get_mut_sphere(&sphere).transform = transformation.clone();
}

#[then(expr = "{word}.transform = {word}")]
fn sphere_transform_equals_identity_matrix(
    world: &mut RayTracerWorld,
    sphere: String,
    transformation: String,
) {
    let sphere = world.get_sphere(&sphere);
    let expected = match transformation.as_str() {
        "identity_matrix" => &Matrix::identity_matrix(),
        _ => world.get_matrix4x4(&transformation),
    };
    assert_eq!(&sphere.transform, expected);
}
