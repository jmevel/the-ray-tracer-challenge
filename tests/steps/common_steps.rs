use cucumber::{given, then, when};
use the_ray_tracer_challenge::Object;

use crate::steps::ray_tracer_world::{RayTracerWorld, Type};

#[given(expr = "{word} ← {word} * {word}")]
#[when(expr = "{word} ← {word} * {word}")]
fn entity_is_entity_multiplied_by_entity(
    world: &mut RayTracerWorld,
    result_entity: String,
    entity1: String,
    entity2: String,
) {
    match (
        world.get_element_type(&entity1),
        world.get_element_type(&entity2),
    ) {
        (Type::Matrix((2, 2)), Type::Matrix(_)) => {
            world.add_matrix2x2(
                result_entity.clone(),
                world.get_matrix2x2(&entity1) * world.get_matrix2x2(&entity2),
            );
        }
        (Type::Matrix((3, 3)), Type::Matrix(_)) => {
            world.add_matrix3x3(
                result_entity.clone(),
                world.get_matrix3x3(&entity1) * world.get_matrix3x3(&entity2),
            );
        }
        (Type::Matrix((4, 4)), Type::Matrix(_)) => {
            world.add_matrix4x4(
                result_entity.clone(),
                world.get_matrix4x4(&entity1) * world.get_matrix4x4(&entity2),
            );
        }
        (Type::Matrix((4, 4)), Type::Point) => {
            world.add_point(
                result_entity.clone(),
                world.get_matrix4x4(&entity1) * world.get_point(&entity2),
            );
        }
        (Type::Matrix((4, 4)), Type::Vector) => {
            world.add_vector(
                result_entity.clone(),
                world.get_matrix4x4(&entity1) * world.get_vector(&entity2),
            );
        }
        _ => panic!("Not supported"),
    }
}

#[when(expr = "{word} ← {word} * {word} * {word}")]
fn entity_is_entity_multiplied_by_entity_multiplied_by_entity(
    world: &mut RayTracerWorld,
    result_entity: String,
    entity1: String,
    entity2: String,
    entity3: String,
) {
    match (
        world.get_element_type(&entity1),
        world.get_element_type(&entity2),
        world.get_element_type(&entity3),
    ) {
        (Type::Matrix((2, 2)), Type::Matrix(_), Type::Matrix(_)) => {
            world.add_matrix2x2(
                result_entity.clone(),
                &(world.get_matrix2x2(&entity1) * world.get_matrix2x2(&entity2))
                    * world.get_matrix2x2(&entity3),
            );
        }
        (Type::Matrix((3, 3)), Type::Matrix(_), Type::Matrix(_)) => {
            world.add_matrix3x3(
                result_entity.clone(),
                &(world.get_matrix3x3(&entity1) * world.get_matrix3x3(&entity2))
                    * world.get_matrix3x3(&entity3),
            );
        }
        (Type::Matrix((4, 4)), Type::Matrix(_), Type::Matrix(_)) => {
            world.add_matrix4x4(
                result_entity.clone(),
                &(world.get_matrix4x4(&entity1) * world.get_matrix4x4(&entity2))
                    * world.get_matrix4x4(&entity3),
            );
        }
        _ => panic!("Not supported"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) = ([a-zA-Z0-9]*)$")]
fn entity_equals_entity(world: &mut RayTracerWorld, entity1: String, entity2: String) {
    match world.get_element_type(&entity1) {
        Type::Point => {
            assert_eq!(world.get_point(&entity1), world.get_point(&entity2));
        }
        Type::Color => {
            assert_eq!(world.get_color(&entity1), world.get_color(&entity2));
        }
        Type::Vector => {
            assert_eq!(world.get_vector(&entity1), world.get_vector(&entity2));
        }
        Type::Matrix((2, 2)) => {
            assert_eq!(world.get_matrix2x2(&entity1), world.get_matrix2x2(&entity2))
        }
        Type::Matrix((3, 3)) => {
            assert_eq!(world.get_matrix3x3(&entity1), world.get_matrix3x3(&entity2))
        }
        Type::Matrix((4, 4)) => {
            assert_eq!(world.get_matrix4x4(&entity1), world.get_matrix4x4(&entity2))
        }
        Type::Intersection => {
            assert_eq!(
                world.get_intersection(&entity1),
                world.get_intersection(&entity2)
            )
        }
        _ => panic!("Not supported"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) != ([a-zA-Z0-9]*)$")]
fn entity_does_not_equal_entity(world: &mut RayTracerWorld, entity1: String, entity2: String) {
    match world.get_element_type(&entity1) {
        Type::Point => {
            assert_ne!(world.get_point(&entity1), world.get_point(&entity2))
        }
        Type::Color => {
            assert_ne!(world.get_color(&entity1), world.get_color(&entity2))
        }
        Type::Vector => {
            assert_ne!(world.get_vector(&entity1), world.get_vector(&entity2))
        }
        Type::Matrix((2, 2)) => {
            assert_ne!(world.get_matrix2x2(&entity1), world.get_matrix2x2(&entity2))
        }
        Type::Matrix((3, 3)) => {
            assert_ne!(world.get_matrix3x3(&entity1), world.get_matrix3x3(&entity2))
        }
        Type::Matrix((4, 4)) => {
            assert_ne!(world.get_matrix4x4(&entity1), world.get_matrix4x4(&entity2))
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(regex = r"^([a-zA-Z0-9_]+) \* ([a-zA-Z0-9_]+) = ([a-zA-Z0-9_]+)$")]
fn entity_multiplied_by_entity_equals_entity(
    world: &mut RayTracerWorld,
    entity1: String,
    entity2: String,
    expected_entity: String,
) {
    match (
        world.get_element_type(&entity1),
        world.get_element_type(&entity2),
        world.get_element_type(&expected_entity),
    ) {
        (Type::Matrix(size), Type::Matrix(_), Type::Matrix(_)) if size == &(2usize, 2usize) => {
            assert_eq!(
                world.get_matrix2x2(&entity1) * world.get_matrix2x2(&entity2),
                *world.get_matrix2x2(&entity1)
            )
        }
        (Type::Matrix(size), Type::Matrix(_), Type::Matrix(_)) if size == &(3usize, 3usize) => {
            assert_eq!(
                world.get_matrix3x3(&entity1) * world.get_matrix3x3(&entity2),
                *world.get_matrix3x3(&entity1)
            )
        }
        (Type::Matrix(size), Type::Matrix(_), Type::Matrix(_)) if size == &(4usize, 4usize) => {
            assert_eq!(
                world.get_matrix4x4(&entity1) * world.get_matrix4x4(&entity2),
                *world.get_matrix4x4(&entity1)
            )
        }
        (Type::Matrix((4, 4)), Type::Vector, Type::Vector) => {
            let transformation = world.get_matrix4x4(&entity1);
            let vector = world.get_vector(&entity2);
            let actual = transformation * vector;
            let expected_entity = world.get_vector(&expected_entity);

            assert_eq!(&actual, expected_entity);
        }
        _ => panic!("Not supported"),
    }
}

#[then(expr = "{word}.count = {int}")]
fn collection_count_equals_count(
    world: &mut RayTracerWorld,
    collection_name: String,
    count: usize,
) {
    match world.get_element_type(&collection_name) {
        Type::IntersectionsCollection => {
            let intersections_collection =
                world.get_intersections_collection(&collection_name).clone();
            if count == 0 {
                assert!(intersections_collection.is_none());
            } else {
                assert_eq!(intersections_collection.unwrap().len(), count);
            }
        }
        _ => panic!("Not supported"),
    }
}

#[then(regex = r#"^([a-zA-Z0-9_]+)\.object = ([a-zA-Z0-9_]+)$"#)]
fn object_of_element_equals_object(world: &mut RayTracerWorld, element: String, object: String) {
    match (
        world.get_element_type(&element),
        world.get_element_type(&object),
    ) {
        (Type::Intersection, Type::Sphere) => {
            let intersection = world.get_intersection(&element).as_ref().unwrap();
            let sphere = world.get_sphere(&object).clone();
            assert_eq!(intersection.object(), &Object::Sphere(sphere));
        }
        (Type::Computations, Type::Sphere) => {
            let computations = world.get_computations(&element);
            let sphere = world.get_sphere(&object).to_owned();
            assert_eq!(computations.object, &Object::Sphere(sphere));
        }
        _ => panic!("Not supported"),
    }
}

#[then(expr = "{word}[{int}].object = {word}")]
fn object_at_index_of_intersections_collection_equals_object(
    world: &mut RayTracerWorld,
    intersections_collection: String,
    index: usize,
    object_name: String,
) {
    let intersection = world
        .get_intersections_collection(&intersections_collection)
        .clone();

    match world.get_element_type(&object_name) {
        Type::Sphere => {
            let sphere = world.get_sphere(&object_name).clone();
            assert_eq!(
                intersection.unwrap()[index].object(),
                &Object::Sphere(sphere)
            );
        }
        _ => panic!("Not supported"),
    }
}

#[then(expr = "{word} is nothing")]
fn entity_is_nothing(world: &mut RayTracerWorld, entity: String) {
    match world.get_element_type(&entity) {
        Type::Intersection => {
            assert!(world.get_intersection(&entity).is_none());
        }
        _ => panic!("Not supported"),
    }
}
