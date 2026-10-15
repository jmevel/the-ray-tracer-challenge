use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Matrix, Plane, Point, Shape, TestShape};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← test_shape\\(\\)")]
fn shape_is_test_shape(world: &mut RayTracerWorld, test_shape_name: String) {
    let test_shape = Shape::TestShape(TestShape::new(None, None));
    world.add_shape(test_shape_name, test_shape);
}

#[given(expr = "{word} ← plane\\(\\)")]
fn shape_is_plane(world: &mut RayTracerWorld, test_shape_name: String) {
    world.add_shape(test_shape_name, Shape::Plane(Plane::new(None, None)));
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

#[given(expr = "set_transform\\({word}, {word})")]
#[when(expr = "set_transform\\({word}, {word})")]
fn set_transform_sphere_transformation(
    world: &mut RayTracerWorld,
    shape: String,
    transformation: String,
) {
    let transformation = world.get_matrix4x4(&transformation).clone();
    let shape = world.get_mut_shape(&shape);
    shape.set_transform(transformation);
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

#[when(expr = "{word} ← normal_at\\({word}, point\\({float}, {float}, {float}))")]
#[when(expr = "{word} ← local_normal_at\\({word}, point\\({float}, {float}, {float}))")]
fn n_normal_at_point(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let shape = world.get_shape(&sphere);
    let point = Point::new_point(x, y, z);
    let normal = shape.normal_at(&point);
    world.add_vector(normal_name, normal.unwrap());
}

#[when(
    expr = "{word} ← normal_at\\({word}, point\\(√{float}\\/{float}, √{float}\\/{float}, √{float}\\/{float}))"
)]
fn n_normal_at_point2(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere: String,
    x_numerator: f32,
    x_denominator: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let x = f32::sqrt(x_numerator) / x_denominator;
    let y = f32::sqrt(y_numerator) / y_denominator;
    let z = f32::sqrt(z_numerator) / z_denominator;
    n_normal_at_point(world, normal_name, sphere, x, y, z);
}

#[when(
    expr = "{word} ← normal_at\\({word}, point\\({float}, √{float}\\/{float}, -√{float}\\/{float}))"
)]
fn n_normal_at_point3(
    world: &mut RayTracerWorld,
    normal_name: String,
    shape: String,
    x: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let y = f32::sqrt(y_numerator) / y_denominator;
    let z = -f32::sqrt(z_numerator) / z_denominator;
    n_normal_at_point(world, normal_name, shape, x, y, z);
}

#[when(expr = "{word} ← {word}.material")]
fn material_is_sphere_material(world: &mut RayTracerWorld, material_name: String, sphere: String) {
    let sphere = world.get_shape(&sphere);
    world.add_material(material_name, sphere.material().clone());
}

#[when(expr = "{word}.material ← {word}")]
fn material_of_sphere_is(world: &mut RayTracerWorld, sphere: String, material: String) {
    let material = world.get_material(&material).clone();
    let Shape::Sphere(sphere) = world.get_mut_shape(&sphere) else {
        panic!("{sphere} is not a sphere")
    };
    sphere.material = material;
}

#[then(expr = "{word}.material = {word}")]
fn material_of_sphere_equals_material(
    world: &mut RayTracerWorld,
    sphere: String,
    material: String,
) {
    let sphere = world.get_shape(&sphere);
    let expected = world.get_material(&material);
    assert_eq!(sphere.material(), expected);
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
