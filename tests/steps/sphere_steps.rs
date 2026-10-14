use std::f32;

use cucumber::{gherkin::Step, given};
use the_ray_tracer_challenge::{Color, Material, Matrix, ReflectionValue, Shape, Sphere};

use crate::steps::ray_tracer_world::RayTracerWorld;

#[given(expr = "{word} ← sphere\\()")]
fn sphere_is(world: &mut RayTracerWorld, sphere_name: String) {
    let sphere = Sphere::new(None, None);
    world.add_shape(sphere_name, Shape::Sphere(sphere));
}

#[given(expr = "{word} ← sphere\\() with:")]
fn sphere_is_sphere_with(world: &mut RayTracerWorld, sphere_name: String, step: &Step) {
    let trim_f32_tuple = |transform_type: &str, field_value: &str| {
        field_value
            .trim_start_matches(transform_type)
            .trim_matches('(')
            .trim_matches(')')
            .split(',')
            .map(|s| s.trim().parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
    };

    if let Some(table) = step.table.as_ref() {
        let mut sphere = Sphere::new(None, None);
        let mut material = Material::default();
        let mut transform: Matrix<4, 4> = Matrix::identity_matrix();
        for row in table.rows.iter() {
            let field_value = &row[1];
            match row[0].as_str() {
                obj if obj.starts_with("material") => {
                    match obj {
                        field_name if field_name.ends_with("color") => {
                            let color_values = trim_f32_tuple("color", field_value);
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
                            let scaling_values = trim_f32_tuple("scaling", field_value);
                            transform = transform.scale(
                                scaling_values[0],
                                scaling_values[1],
                                scaling_values[2],
                            );
                        }
                        transform_type if transform_type.starts_with("translation") => {
                            let translation_values = trim_f32_tuple("translation", field_value);
                            transform = transform.translate(
                                translation_values[0],
                                translation_values[1],
                                translation_values[2],
                            );
                        }
                        _ => panic!("Not implemented"),
                    }
                    sphere.transform = transform;
                }
                _ => panic!("Not implemented"),
            }
        }
        world.add_shape(sphere_name, Shape::Sphere(sphere));
    } else {
        panic!("Missing table");
    }
}
