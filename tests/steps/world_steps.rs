use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Object, World};

use crate::steps::ray_tracer_world::{RayTracerWorld, Type};

#[given(expr = "{word} ← world\\()")]
fn world_is_new_world(world: &mut RayTracerWorld, world_name: String) {
    world.add_world(world_name, World::new());
}

#[given(expr = "{word} ← default_world\\()")]
#[when(expr = "{word} ← default_world\\()")]
fn world_is_default_world(world: &mut RayTracerWorld, world_name: String) {
    world.add_world(world_name, World::default());
}

#[when(expr = "{word} ← intersect_world\\({word}, {word})")]
fn intersections_is_world_intersected_with_ray(
    world: &mut RayTracerWorld,
    intersections_name: String,
    _world: String,
    ray: String,
) {
    let scene_world = world.get_world();
    let ray = world.get_ray(&ray);
    let intersections = scene_world.intersect(&ray).unwrap();

    world.add_intersections_collection(intersections_name, intersections);
}

#[then(expr = "{word} contains no objects")]
fn world_contains_no_objects(world: &mut RayTracerWorld, _world_name: String) {
    assert!(world.get_world().elements.is_empty());
}

#[then(expr = "{word} has no light source")]
fn world_has_no_light_source(world: &mut RayTracerWorld, _world_name: String) {
    assert!(world.get_world().light.is_none());
}

#[then(expr = "{word}.light = {word}")]
fn world_light_equals_light(world: &mut RayTracerWorld, _world_name: String, light: String) {
    let expected = world.get_point_light(&light);
    let scene_world = world.get_world();
    assert_eq!(scene_world.light.as_ref().unwrap(), expected);
}

#[then(expr = "{word} contains {word}")]
fn world_contains_element(world: &mut RayTracerWorld, _world_name: String, element: String) {
    let scene_world = world.get_world();
    match world.get_element_type(&element) {
        Type::Sphere => {
            let expected = world.get_sphere(&element);
            assert!(scene_world.elements.iter().any(|e| {
                let Object::Sphere(actual) = e;
                actual.material == expected.material && actual.transform == expected.transform
            }));
        }
        _ => panic!("Not implemented"),
    }
}
