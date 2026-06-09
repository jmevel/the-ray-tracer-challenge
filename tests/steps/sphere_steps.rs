use std::f32;

use cucumber::{given, then, when};
use the_ray_tracer_challenge::{Material, Matrix, Point, Sphere};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← sphere\\()")]
fn sphere_is(world: &mut RayTracerWorld, sphere_name: String) {
    let sphere = Sphere::new(None);
    world.add_sphere(sphere_name, sphere);
}

#[given(expr = "set_transform\\({word}, translation\\({float}, {float}, {float}))")]
fn set_transform_s_translation(world: &mut RayTracerWorld, sphere: String, x: f32, y: f32, z: f32) {
    let sphere = world.get_mut_sphere(&sphere);
    let transformation = Matrix::new_translation(x, y, z);
    sphere.transform = transformation;
}

#[given(expr = "{word} ← scaling\\({float}, {float}, {float}) * rotation_z\\(π\\/{float})")]
fn transformation_is_scaling_and_rotation_z(
    world: &mut RayTracerWorld,
    transformation_name: String,
    x: f32,
    y: f32,
    z: f32,
    denominator: f32,
) {
    let transformation: Matrix<4, 4> =
        Matrix::new_rotation_z(f32::consts::PI / denominator).scale(x, y, z);
    world.add_matrix4x4(transformation_name, transformation);
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
#[given(expr = "set_transform\\({word}, {word})")]
fn set_transform_s_t(world: &mut RayTracerWorld, sphere: String, transformation: String) {
    let transformation = world.get_matrix4x4(&transformation);
    world.get_mut_sphere(&sphere).transform = transformation.clone();
}

#[when(expr = "{word} ← normal_at\\({word}, point\\({float}, {float}, {float}))")]
fn n_normal_at_point(
    world: &mut RayTracerWorld,
    normal_name: String,
    sphere: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let sphere = world.get_sphere(&sphere);
    let point = Point::new_point(x, y, z);
    let normal = sphere.normal_at(&point);
    world.add_vector(normal_name, normal);
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
    sphere: String,
    x: f32,
    y_numerator: f32,
    y_denominator: f32,
    z_numerator: f32,
    z_denominator: f32,
) {
    let y = f32::sqrt(y_numerator) / y_denominator;
    let z = -f32::sqrt(z_numerator) / z_denominator;
    n_normal_at_point(world, normal_name, sphere, x, y, z);
}

#[when(expr = "{word} ← {word}.material")]
fn material_is_sphere_material(world: &mut RayTracerWorld, material_name: String, sphere: String) {
    let sphere = world.get_sphere(&sphere);
    world.add_material(material_name, sphere.material.clone());
}

#[when(expr = "{word}.material ← {word}")]
fn material_of_sphere_is(world: &mut RayTracerWorld, sphere: String, material: String) {
    let material = world.get_material(&material).clone();
    let sphere = world.get_mut_sphere(&sphere);
    sphere.material = material;
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

#[then(expr = "{word} = material\\()")]
fn material_equals_default_material(world: &mut RayTracerWorld, material: String) {
    let actual = world.get_material(&material);
    assert_eq!(actual, &Material::default());
}

#[then(expr = "{word}.material = {word}")]
fn material_of_sphere_equals_material(
    world: &mut RayTracerWorld,
    sphere: String,
    material: String,
) {
    let sphere = world.get_sphere(&sphere);
    let expected = world.get_material(&material);
    assert_eq!(&sphere.material, expected);
}
