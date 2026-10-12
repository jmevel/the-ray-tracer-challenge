use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Matrix, Shape, TestShape};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← test_shape\\(\\)")]
fn shape_is_test_shape(world: &mut RayTracerWorld, test_shape_name: String) {
    let test_shape = Shape::TestShape(TestShape::new(None, None));
    world.add_shape(test_shape_name, test_shape);
}

#[given(expr = "set_transform\\({word}, translation\\({float}, {float}, {float}))")]
#[when(expr = "set_transform\\({word}, translation\\({float}, {float}, {float}))")]
fn set_transform_translation_to_shape(
    world: &mut RayTracerWorld,
    shape: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let shape = world.get_mut_shape(&shape);
    shape.set_transform(Matrix::new_translation(x, y, z));
}

#[when(expr = "set_transform\\({word}, scaling\\({int}, {int}, {int}))")]
fn set_transform_scaling_to_shape(
    world: &mut RayTracerWorld,
    shape: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let shape = world.get_mut_shape(&shape);
    shape.set_transform(Matrix::new_scaling(x, y, z));
}

#[when(expr = "{word} ← intersect\\({word}, {word})")]
fn intersection_is_intersect_of_shape_and_ray(
    world: &mut RayTracerWorld,
    intersect_name: String,
    sphere_name: String,
    ray_name: String,
) -> Result<(), String> {
    let shape = world.get_shape(&sphere_name);
    let ray = world.get_ray(&ray_name);
    let intersections = shape.intersect(ray);
    world.add_intersections_collection(intersect_name, intersections?);
    Ok(())
}

#[then(expr = "{word}.transform = translation\\({float}, {float}, {float})")]
fn shape_transform_equals_translation(
    world: &mut RayTracerWorld,
    shape: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let actual = world.get_shape(&shape);
    let expected = Matrix::new_translation(x, y, z);
    assert_eq!(actual.transform(), &expected);
}
