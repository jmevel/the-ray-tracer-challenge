use cucumber::{given, then, when};

use crate::steps::ray_tracer_world::{RayTracerWorld, Type};

#[given(expr = "{word} ← {word} * {word}")]
fn entity_is_entity_multiplied_by_entity(
    world: &mut RayTracerWorld,
    result_entity: String,
    entity1: String,
    entity2: String,
) {
    match (
        world.get_element_type(&result_entity),
        world.get_element_type(&entity1),
        world.get_element_type(&entity2),
    ) {
        (Type::Matrix(size), Type::Matrix(_), Type::Matrix(_)) if size == &(2usize, 2usize) => {
            world.add_matrix2x2(
                result_entity.clone(),
                world.get_matrix2x2(&entity1) * world.get_matrix2x2(&entity2),
            );
        }
        (Type::Matrix(size), Type::Matrix(_), Type::Matrix(_)) if size == &(3usize, 3usize) => {
            world.add_matrix3x3(
                result_entity.clone(),
                world.get_matrix3x3(&entity1) * world.get_matrix3x3(&entity2),
            );
        }
        (Type::Matrix(size), Type::Matrix(_), Type::Matrix(_)) if size == &(4usize, 4usize) => {
            world.add_matrix4x4(
                result_entity.clone(),
                world.get_matrix4x4(&entity1) * world.get_matrix4x4(&entity2),
            );
        }
        _ => panic!("Not supported"),
    }
}

#[when(expr = "{word} ← {word} * {word}")]
fn entity_is_entity_multiplied_by_entity2(
    world: &mut RayTracerWorld,
    result_entity: String,
    entity1: String,
    entity2: String,
) {
    match (
        world.get_element_type(&entity1),
        world.get_element_type(&entity2),
    ) {
        (Type::Matrix(size), Type::Matrix(_)) if size == &(2usize, 2usize) => {
            world.add_matrix2x2(
                result_entity.clone(),
                world.get_matrix2x2(&entity1) * world.get_matrix2x2(&entity2),
            );
        }
        (Type::Matrix(size), Type::Matrix(_)) if size == &(3usize, 3usize) => {
            world.add_matrix3x3(
                result_entity.clone(),
                world.get_matrix3x3(&entity1) * world.get_matrix3x3(&entity2),
            );
        }
        (Type::Matrix(size), Type::Matrix(_)) if size == &(4usize, 4usize) => {
            world.add_matrix4x4(
                result_entity.clone(),
                world.get_matrix4x4(&entity1) * world.get_matrix4x4(&entity2),
            );
        }
        (Type::Matrix((4, 4)), Type::Tuple) => {
            world.add_tuple(
                result_entity.clone(),
                world.get_matrix4x4(&entity1) * world.get_tuple(&entity2),
            );
        }
        _ => panic!("Not supported"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) = ([a-zA-Z0-9]*)$")]
fn entity_equals_entity(world: &mut RayTracerWorld, entity1: String, entity2: String) {
    match world.get_element_type(&entity1) {
        Type::Tuple => {
            assert_eq!(world.get_tuple(&entity1), world.get_tuple(&entity2));
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
        _ => panic!("no matrix with given size"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) != ([a-zA-Z0-9]*)$")]
fn entity_does_not_equal_entity(world: &mut RayTracerWorld, entity1: String, entity2: String) {
    match world.get_element_type(&entity1) {
        Type::Tuple => {
            assert!(!(world.get_tuple(&entity1) == world.get_tuple(&entity2)));
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
        (Type::Matrix((4, 4)), Type::Tuple, Type::Tuple) => {
            let transformation = world.get_matrix4x4(&entity1);
            let vector = world.get_tuple(&entity2);
            let actual = transformation * vector;
            let expected_entity = world.get_tuple(&expected_entity);

            assert_eq!(&actual, expected_entity);
        }
        _ => panic!("Not supported"),
    }
}
