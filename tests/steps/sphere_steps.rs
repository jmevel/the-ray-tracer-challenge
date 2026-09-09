use std::f32;

use cucumber::{gherkin::Step, given, then, when};
use the_ray_tracer_challenge::{Color, Material, Matrix, Point, ReflectionValue, Sphere};

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

#[given(expr = "{word} ← sphere\\() with:")]
fn sphere_is_sphere_with(world: &mut RayTracerWorld, sphere_name: String, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        let mut sphere = Sphere::new(None);
        let mut material = Material::default();
        let mut transform: Matrix<4, 4> = Matrix::identity_matrix();
        for row in table.rows.iter() {
            let field_value = &row[1];
            match row[0].as_str() {
                obj if obj.starts_with("material") => {
                    match obj {
                        field_name if field_name.ends_with("color") => {
                            let color_values = field_value
                                .trim_matches('(')
                                .trim_matches(')')
                                .split(',')
                                .map(|s| s.trim().parse::<f32>().unwrap())
                                .collect::<Vec<f32>>();
                            material.color =
                                Color::new_color(color_values[0], color_values[1], color_values[2]);
                        }
                        field_name if field_name.ends_with("diffuse") => {
                            material.diffuse =
                                ReflectionValue::new(field_value.parse::<f32>().unwrap());
                        }
                        field_name if field_name.ends_with("specular") => {
                            material.specular =
                                ReflectionValue::new(field_value.parse::<f32>().unwrap());
                        }
                        _ => panic!("Not implemented"),
                    }
                    sphere.material = material;
                }
                obj if obj.starts_with("transform") => {
                    match field_value {
                        transform_type if transform_type.starts_with("scaling") => {
                            let scaling_values = field_value
                                .trim_start_matches("scaling")
                                .trim_matches('(')
                                .trim_matches(')')
                                .split(',')
                                .map(|s| s.trim().parse::<f32>().unwrap())
                                .collect::<Vec<f32>>();

                            transform = transform.scale(
                                scaling_values[0],
                                scaling_values[1],
                                scaling_values[2],
                            );
                        }
                        _ => panic!("Not implemented"),
                    }
                    sphere.transform = transform;
                }
                _ => panic!("Not implemented"),
            }
        }
        world.add_sphere(sphere_name, sphere);
    } else {
        panic!("Missing table");
    }
}

#[given(expr = "set_transform\\({word}, {word})")]
#[when(expr = "set_transform\\({word}, {word})")]
fn set_transform_s_t(world: &mut RayTracerWorld, sphere: String, transformation: String) {
    let transformation = world.get_matrix4x4(&transformation);
    world.get_mut_sphere(&sphere).transform = transformation.clone();
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
    world.add_material(material_name, sphere.material().clone());
}

#[when(expr = "{word}.material ← {word}")]
fn material_of_sphere_is(world: &mut RayTracerWorld, sphere: String, material: String) {
    let material = world.get_material(&material).clone();
    let sphere = world.get_mut_sphere(&sphere);
    sphere.material = material;
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
    assert_eq!(sphere.material(), expected);
}
