use cucumber::{then, when};
use the_ray_tracer_challenge::PointLight;

use crate::steps::ray_tracer_world::RayTracerWorld;

#[when(expr = "{word} ← point_light\\({word}, {word})")]
fn point_light_is_position_and_intensity(
    world: &mut RayTracerWorld,
    point_light_name: String,
    position: String,
    intensity: String,
) {
    let position = world.get_point(&position);
    let intensity = world.get_color(&intensity);
    let point_light = PointLight::new(position.clone(), intensity.clone());
    world.add_point_light(point_light_name, point_light);
}

#[then(expr = "{word}.position = {word}")]
fn light_position_equals_position(
    world: &mut RayTracerWorld,
    point_light: String,
    position: String,
) {
    let point_light = world.get_point_light(&point_light);
    let expected = world.get_point(&position);
    assert_eq!(point_light.position(), expected);
}

#[then(expr = "{word}.intensity = {word}")]
fn light_intensity_equals_position(
    world: &mut RayTracerWorld,
    point_light: String,
    intensity: String,
) {
    let point_light = world.get_point_light(&point_light);
    let expected = world.get_color(&intensity);
    assert_eq!(point_light.intensity(), expected);
}
