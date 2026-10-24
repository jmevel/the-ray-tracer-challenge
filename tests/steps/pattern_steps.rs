use cucumber::{given, then};
use the_ray_tracer_challenge::StripePattern;

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← stripe_pattern\\({word}, {word})")]
fn value_is_integer(
    world: &mut RayTracerWorld,
    pattern_name: String,
    color_1: String,
    color_2: String,
) {
    let color_1 = world.get_color(&color_1);
    let color_2 = world.get_color(&color_2);
    let pattern = StripePattern::new(color_1.clone(), color_2.clone());
    world.add_pattern(pattern_name, pattern);
}

#[then(expr = "{word}.a = {word}")]
fn pattern_color_1_is_color(world: &mut RayTracerWorld, pattern: String, expected_color: String) {
    let pattern = world.get_pattern(&pattern);
    let expected_color = world.get_color(&expected_color);
    assert_eq!(pattern.color_1(), expected_color);
}

#[then(expr = "{word}.b = {word}")]
fn pattern_color_2_is_color(world: &mut RayTracerWorld, pattern: String, expected_color: String) {
    let pattern = world.get_pattern(&pattern);
    let expected_color = world.get_color(&expected_color);
    assert_eq!(pattern.color_2(), expected_color);
}
