use cucumber::{given, then, when};
use the_ray_tracer_challenge::Object;

use crate::steps::ray_tracer_world::{ElementType, RayTracerWorld};

#[given(expr = "{word} ← {word} * {word}")]
#[when(expr = "{word} ← {word} * {word}")]
fn entity_is_entity_multiplied_by_entity(
    world: &mut RayTracerWorld,
    result_entity: String,
    entity1: String,
    entity2: String,
) {
    match (world.get_element(&entity1), world.get_element(&entity2)) {
        (ElementType::Matrix2x2(entity1), ElementType::Matrix2x2(entity2)) => {
            world.add_matrix2x2(result_entity.clone(), entity1 * entity2);
        }
        (ElementType::Matrix3x3(entity1), ElementType::Matrix3x3(entity2)) => {
            world.add_matrix3x3(result_entity.clone(), entity1 * entity2);
        }
        (ElementType::Matrix4x4(entity1), ElementType::Matrix4x4(entity2)) => {
            world.add_matrix4x4(result_entity.clone(), entity1 * entity2);
        }
        (ElementType::Matrix4x4(matrix), ElementType::Point(point)) => {
            world.add_point(result_entity.clone(), matrix * point);
        }
        (ElementType::Matrix4x4(matrix), ElementType::Vector(vector)) => {
            world.add_vector(result_entity.clone(), matrix * vector);
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
        world.get_element(&entity1),
        world.get_element(&entity2),
        world.get_element(&entity3),
    ) {
        (
            ElementType::Matrix2x2(entity1),
            ElementType::Matrix2x2(entity2),
            ElementType::Matrix2x2(entity3),
        ) => {
            world.add_matrix2x2(result_entity.clone(), &(entity1 * entity2) * entity3);
        }
        (
            ElementType::Matrix3x3(entity1),
            ElementType::Matrix3x3(entity2),
            ElementType::Matrix3x3(entity3),
        ) => {
            world.add_matrix3x3(result_entity.clone(), &(entity1 * entity2) * entity3);
        }
        (
            ElementType::Matrix4x4(entity1),
            ElementType::Matrix4x4(entity2),
            ElementType::Matrix4x4(entity3),
        ) => {
            world.add_matrix4x4(result_entity.clone(), &(entity1 * entity2) * entity3);
        }
        _ => panic!("Not supported"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) = ([a-zA-Z0-9]*)$")]
fn entity_equals_entity(world: &mut RayTracerWorld, entity1: String, entity2: String) {
    match (world.get_element(&entity1), world.get_element(&entity2)) {
        (ElementType::Point(entity1), ElementType::Point(entity2)) => {
            assert_eq!(entity1, entity2);
        }
        (ElementType::Color(entity1), ElementType::Color(entity2)) => {
            assert_eq!(entity1, entity2);
        }
        (ElementType::Vector(entity1), ElementType::Vector(entity2)) => {
            assert_eq!(entity1, entity2);
        }
        (ElementType::Matrix2x2(entity1), ElementType::Matrix2x2(entity2)) => {
            assert_eq!(entity1, entity2)
        }
        (ElementType::Matrix3x3(entity1), ElementType::Matrix3x3(entity2)) => {
            assert_eq!(entity1, entity2)
        }
        (ElementType::Matrix4x4(entity1), ElementType::Matrix4x4(entity2)) => {
            assert_eq!(entity1, entity2)
        }
        (ElementType::Intersection(entity1), ElementType::Intersection(entity2)) => {
            assert_eq!(entity1, entity2)
        }
        _ => panic!("Not supported"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) != ([a-zA-Z0-9]*)$")]
fn entity_does_not_equal_entity(world: &mut RayTracerWorld, entity1: String, entity2: String) {
    match (world.get_element(&entity1), world.get_element(&entity2)) {
        (ElementType::Point(entity1), ElementType::Point(entity2)) => {
            assert_ne!(entity1, entity2);
        }
        (ElementType::Color(entity1), ElementType::Color(entity2)) => {
            assert_ne!(entity1, entity2);
        }
        (ElementType::Vector(entity1), ElementType::Vector(entity2)) => {
            assert_ne!(entity1, entity2);
        }
        (ElementType::Matrix2x2(entity1), ElementType::Matrix2x2(entity2)) => {
            assert_ne!(entity1, entity2)
        }
        (ElementType::Matrix3x3(entity1), ElementType::Matrix3x3(entity2)) => {
            assert_ne!(entity1, entity2)
        }
        (ElementType::Matrix4x4(entity1), ElementType::Matrix4x4(entity2)) => {
            assert_ne!(entity1, entity2)
        }
        (ElementType::Intersection(entity1), ElementType::Intersection(entity2)) => {
            assert_ne!(entity1, entity2)
        }
        _ => panic!("Not supported"),
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
        world.get_element(&entity1),
        world.get_element(&entity2),
        world.get_element(&expected_entity),
    ) {
        (
            ElementType::Matrix2x2(entity1),
            ElementType::Matrix2x2(entity2),
            ElementType::Matrix2x2(expected_entity),
        ) => {
            assert_eq!(&(entity1 * entity2), expected_entity);
        }
        (
            ElementType::Matrix3x3(entity1),
            ElementType::Matrix3x3(entity2),
            ElementType::Matrix3x3(expected_entity),
        ) => {
            assert_eq!(&(entity1 * entity2), expected_entity);
        }
        (
            ElementType::Matrix4x4(entity1),
            ElementType::Matrix4x4(entity2),
            ElementType::Matrix4x4(expected_entity),
        ) => {
            assert_eq!(&(entity1 * entity2), expected_entity);
        }
        (
            ElementType::Matrix4x4(entity1),
            ElementType::Vector(entity2),
            ElementType::Vector(expected_entity),
        ) => {
            assert_eq!(&(entity1 * entity2), expected_entity);
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
    match world.get_element(&collection_name) {
        ElementType::IntersectionsCollection(intersections) => {
            if count == 0 {
                assert!(intersections.is_none());
            } else {
                assert_eq!(intersections.as_ref().unwrap().len(), count);
            }
        }
        _ => panic!("Not supported"),
    }
}

#[then(regex = r#"^([a-zA-Z0-9_]+)\.object = ([a-zA-Z0-9_]+)$"#)]
fn object_of_element_equals_object(world: &mut RayTracerWorld, element: String, expected: String) {
    match (world.get_element(&element), world.get_element(&expected)) {
        (ElementType::Intersection(element), ElementType::Sphere(expected)) => {
            assert_eq!(
                element.as_ref().unwrap().object(),
                &Object::Sphere(expected.to_owned())
            );
        }
        (ElementType::Computations(element), ElementType::Sphere(expected)) => {
            assert_eq!(
                element.as_computations().object,
                &Object::Sphere(expected.to_owned())
            );
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

    match world.get_element(&object_name) {
        ElementType::Sphere(sphere) => {
            assert_eq!(
                intersection.unwrap()[index].object(),
                &Object::Sphere(sphere.to_owned())
            );
        }
        _ => panic!("Not supported"),
    }
}

#[then(expr = "{word} is nothing")]
fn entity_is_nothing(world: &mut RayTracerWorld, entity: String) {
    match world.get_element(&entity) {
        ElementType::Intersection(intersection) => {
            assert!(intersection.is_none());
        }
        _ => panic!("Not supported"),
    }
}
