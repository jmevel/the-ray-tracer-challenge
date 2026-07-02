use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Color, Object, Point, PointLight, World};

use crate::steps::{
    ray_tracer_world::{ElementType, RayTracerWorld},
    utils::Nth,
};

#[given(expr = "{word} ← world\\()")]
fn world_is_new_world(world: &mut RayTracerWorld, world_name: String) {
    world.add_world(world_name, World::new());
}

#[given(expr = "{word} ← default_world\\()")]
#[when(expr = "{word} ← default_world\\()")]
fn world_is_default_world(world: &mut RayTracerWorld, world_name: String) {
    world.add_world(world_name, World::default());
}

#[given(expr = "{word} ← the {nth} object in {word}")]
fn shape_is_the_nth_object_in_world(
    world: &mut RayTracerWorld,
    shape_name: String,
    nth: Nth,
    world_name: String,
) {
    let scene_world = world.get_mut_world(&world_name);
    let Object::Sphere(shape) = scene_world.elements[nth as usize];
    world.add_sphere(shape_name, shape);
}

#[given(
    expr = "{word}.light ← point_light\\(point\\({float}, {float}, {float}), color\\({float}, {float}, {float}))"
)]
fn light_of_world_is(
    world: &mut RayTracerWorld,
    world_name: String,
    x: f32,
    y: f32,
    z: f32,
    red: f32,
    green: f32,
    blue: f32,
) {
    let position = Point::new_point(x, y, z);
    let intensity = Color::new_color(red, green, blue);
    let scene_world = world.get_mut_world(&world_name);
    scene_world.lights = Some(vec![PointLight::new(position, intensity)]);
}

#[when(expr = "{word} ← intersect_world\\({word}, {word})")]
fn intersections_is_world_intersected_with_ray(
    world: &mut RayTracerWorld,
    intersections_name: String,
    world_name: String,
    ray: String,
) {
    let scene_world = world.get_world(&world_name);
    let ray = world.get_ray(&ray);
    let intersections = scene_world.intersect(&ray).unwrap();

    world.add_intersections_collection(intersections_name, intersections);
}

#[when(expr = "{word} ← shade_hit\\({word}, {word})")]
fn color_is_shade_it_result(
    world: &mut RayTracerWorld,
    color_name: String,
    world_name: String,
    computations: String,
) {
    let scene_world = world.get_world(&world_name);
    let computations = world.get_computations(&computations);
    let color = scene_world.shade_hit(computations);
    world.add_color(color_name, color);
}

#[when(expr = "{word} ← color_at\\({word}, {word})")]
fn color_is_color_at(
    world: &mut RayTracerWorld,
    color_name: String,
    world_name: String,
    ray: String,
) {
    let scene_world = world.get_world(&world_name);
    let ray = world.get_ray(&ray);
    world.add_color(color_name, scene_world.color_at(ray).unwrap());
}

#[then(expr = "{word} contains no objects")]
fn world_contains_no_objects(world: &mut RayTracerWorld, world_name: String) {
    assert!(world.get_world(&world_name).elements.is_empty());
}

#[then(expr = "{word} has no light source")]
fn world_has_no_light_source(world: &mut RayTracerWorld, world_name: String) {
    assert!(world.get_world(&world_name).lights.is_none());
}

#[then(expr = "{word}.light = {word}")]
fn world_light_equals_light(world: &mut RayTracerWorld, world_name: String, light: String) {
    let expected = world.get_point_light(&light);
    let scene_world = world.get_world(&world_name);
    assert_eq!(
        scene_world.lights.as_ref().unwrap().first().unwrap(),
        expected
    );
}

#[then(expr = "{word} contains {word}")]
fn world_contains_element(world: &mut RayTracerWorld, world_name: String, element: String) {
    let scene_world = world.get_world(&world_name);
    match world.get_element(&element) {
        ElementType::Sphere(expected) => {
            assert!(scene_world.elements.iter().any(|e| {
                let Object::Sphere(actual) = e;
                actual.material == expected.material && actual.transform == expected.transform
            }));
        }
        _ => panic!("Not implemented"),
    }
}
