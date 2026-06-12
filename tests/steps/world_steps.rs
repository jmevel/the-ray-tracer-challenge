use cucumber::{given, then};
use the_ray_tracer_challenge::World;

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← world\\()")]
fn world_is_new_world(world: &mut RayTracerWorld, world_name: String) {
    world.add_world(world_name, World::new());
}

#[then(expr = "{word} contains no objects")]
fn world_contains_no_objects(world: &mut RayTracerWorld, _world_name: String) {
    assert!(world.get_world().elements.is_empty());
}

#[then(expr = "{word} has no light source")]
fn world_has_no_light_source(world: &mut RayTracerWorld, _world_name: String) {
    assert!(world.get_world().light.is_none());
}
