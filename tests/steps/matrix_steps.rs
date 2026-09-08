use crate::RayTracerWorld;
use crate::steps::ray_tracer_world::ElementType;
use cucumber::gherkin::Step;
use cucumber::{given, then};
use the_ray_tracer_challenge::{Matrix, Point, Tuple, Vector};

#[given(expr = "the following 2x2 matrix {word}:")]
fn the_following_2x2_matrix(world: &mut RayTracerWorld, matrix_name: String, step: &Step) {
    world.add_matrix2x2(matrix_name, get_matrix::<2, 2>(step));
}

#[given(expr = "the following 3x3 matrix {word}:")]
fn the_following_3x3_matrix(world: &mut RayTracerWorld, matrix_name: String, step: &Step) {
    world.add_matrix3x3(matrix_name, get_matrix::<3, 3>(step));
}

#[given(expr = "the following 4x4 matrix {word}:")]
fn the_following_4x4_matrix(world: &mut RayTracerWorld, matrix_name: String, step: &Step) {
    world.add_matrix4x4(matrix_name, get_matrix::<4, 4>(step));
}

#[given(expr = "the following matrix {word}:")]
fn the_following_matrix(world: &mut RayTracerWorld, matrix: String, step: &Step) {
    match get_matrix_size(step) {
        2 => {
            the_following_2x2_matrix(world, matrix, step);
        }
        3 => {
            the_following_3x3_matrix(world, matrix, step);
        }
        4 => {
            the_following_4x4_matrix(world, matrix, step);
        }
        _ => panic!("matrix size not supported"),
    }
}

#[given(expr = "{word} ← transpose\\(identity_matrix)")]
fn transpose_identity_matrix(world: &mut RayTracerWorld, matrix: String) {
    let transposed_identity_matrix: Matrix<4, 4> = Matrix::identity_matrix().transpose();
    world.add_matrix4x4(matrix, transposed_identity_matrix);
}

#[given(expr = "{word} ← submatrix\\({word}, {int}, {int})")]
fn matrix_is_submatrix(
    world: &mut RayTracerWorld,
    new_matrix_name: String,
    initial_matrix_name: String,
    row_idx: usize,
    col_idx: usize,
) {
    let initial_matrix = match world.get_element(&initial_matrix_name) {
        ElementType::Matrix3x3(matrix) => matrix,
        ElementType::Matrix2x2(_) | ElementType::Matrix4x4(_) => {
            panic!("Only supported on 3x3 matrices")
        }
        _ => panic!("no matrix with given size"),
    };

    let new_matrix: Matrix<2, 2> = initial_matrix.submatrix(row_idx, col_idx);
    world.add_matrix2x2(new_matrix_name, new_matrix);
}

#[given(expr = "{word} ← inverse\\({word})")]
fn matrix_is_inverse_of_matrix(
    world: &mut RayTracerWorld,
    new_matrix_name: String,
    initial_matrix_name: String,
) {
    match world.get_element(&initial_matrix_name) {
        ElementType::Matrix2x2(_) => panic!("can't invert a 2x2 matrix"),
        ElementType::Matrix3x3(matrix) => {
            world.add_matrix3x3(new_matrix_name, matrix.invert().unwrap());
        }
        ElementType::Matrix4x4(matrix) => {
            world.add_matrix4x4(new_matrix_name, matrix.invert().unwrap());
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word}[{int},{int}] = {float}")]
fn matrix_index_equals(
    world: &mut RayTracerWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    value: f32,
) {
    match world.get_element(&matrix) {
        ElementType::Matrix2x2(matrix) => {
            assert_eq!(matrix.data[row_idx][col_idx], value)
        }
        ElementType::Matrix3x3(matrix) => {
            assert_eq!(matrix.data[row_idx][col_idx], value)
        }
        ElementType::Matrix4x4(matrix) => {
            assert_eq!(matrix.data[row_idx][col_idx], value)
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} * {word} is the following {int}x{int} matrix:")]
fn matrix_multiplied_by_matrix_is_the_following_matrix(
    world: &mut RayTracerWorld,
    matrix1: String,
    matrix2: String,
    row_size: usize,
    col_size: usize,
    step: &Step,
) {
    match (row_size, col_size) {
        (2, 2) | (3, 3) => panic!("step definition not implemented (not needed)"),
        (4, 4) => {
            let matrix1 = world.get_matrix4x4(&matrix1);
            let matrix2 = world.get_matrix4x4(&matrix2);

            let expected = get_matrix::<4, 4>(step);
            let actual = matrix1 * matrix2;

            assert_eq!(actual, expected);
        }
        _ => panic!("no matrix with given size"),
    }
}

// Then A * B is the following 4x4 matrix:
#[then(
    regex = r"^([a-zA-Z0-9]*) is the following ((?:-?\d+)|(?:\d+))x((?:-?\d+)|(?:\d+)) matrix:$"
)]
fn matrix_is_the_following_matrix(
    world: &mut RayTracerWorld,
    matrix: String,
    row_size: usize,
    col_size: usize,
    step: &Step,
) {
    match (row_size, col_size) {
        (2, 2) => panic!("step definition not implemented (not needed)"),
        (3, 3) => {
            let matrix = world.get_matrix3x3(&matrix);
            let expected = get_matrix::<3, 3>(step);
            assert_eq!(matrix, &expected);
        }
        (4, 4) => {
            let matrix = world.get_matrix4x4(&matrix);
            let expected = get_matrix::<4, 4>(step);
            assert_eq!(matrix, &expected);
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(
    // A * b = tuple(18, 24, 33, 1)
    regex = r"^([a-zA-Z0-9]*) \* ([a-zA-Z0-9]*) = tuple\(([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?)), ([+-]?(?:inf|NaN|(?:\d+|\d+\.\d*|\d*\.\d+)(?:[eE][+-]?\d+)?))\)$"
)]
fn matrix_multiplied_by_tuple_equals_tuple(
    world: &mut RayTracerWorld,
    matrix: String,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let matrix = world.get_matrix4x4(&matrix);
    match world.get_element(&tuple) {
        ElementType::Point(point) => {
            let expected = Point::new(x, y, z, w);
            assert_eq!(matrix * point, expected);
        }
        ElementType::Vector(vector) => {
            let expected = Vector::new(x, y, z, w);
            assert_eq!(matrix * vector, expected);
        }
        _ => panic!("Not implemented"),
    };
}

#[then(expr = "transpose\\({word}) is the following matrix:")]
fn transpose_matrix_is_the_following_matrix(
    world: &mut RayTracerWorld,
    matrix: String,
    step: &Step,
) {
    match world.get_element(&matrix) {
        ElementType::Matrix2x2(matrix) => assert_eq!(matrix.transpose(), get_matrix::<2, 2>(step)),
        ElementType::Matrix3x3(matrix) => assert_eq!(matrix.transpose(), get_matrix::<3, 3>(step)),
        ElementType::Matrix4x4(matrix) => assert_eq!(matrix.transpose(), get_matrix::<4, 4>(step)),
        _ => panic!("no matrix with given size"),
    }
}

#[then(regex = r#"^([a-zA-Z0-9_]+) = identity_matrix$"#)]
fn matrix_equals_identity_matrix(world: &mut RayTracerWorld, matrix: String) {
    let matrix = world.get_matrix4x4(&matrix);
    let identity_matrix: Matrix<4, 4> = Matrix::identity_matrix();
    assert_eq!(&identity_matrix, matrix);
}

#[then(expr = "determinant\\({word}) = {int}")]
fn determinant_equals(world: &mut RayTracerWorld, matrix: String, determinant: f32) {
    match world.get_element(&matrix) {
        ElementType::Matrix2x2(matrix) => assert_eq!(matrix.determinant(), f32::from(determinant)),
        ElementType::Matrix3x3(matrix) => assert_eq!(matrix.determinant(), f32::from(determinant)),
        ElementType::Matrix4x4(matrix) => assert_eq!(matrix.determinant(), f32::from(determinant)),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "submatrix\\({word}, {int}, {int}) is the following {int}x{int} matrix:")]
fn submatrix_is_the_following_matrix(
    world: &mut RayTracerWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    _row_size: usize,
    _col_size: usize,
    step: &Step,
) {
    match world.get_element(&matrix) {
        ElementType::Matrix2x2(_) => panic!("can't make a submatrix of a 2x2 matrix"),
        ElementType::Matrix3x3(matrix) => {
            assert_eq!(matrix.submatrix(row_idx, col_idx), get_matrix::<2, 2>(step))
        }
        ElementType::Matrix4x4(matrix) => {
            assert_eq!(matrix.submatrix(row_idx, col_idx), get_matrix::<3, 3>(step))
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "minor\\({word}, {int}, {int}) = {int}")]
fn minor_equals(
    world: &mut RayTracerWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    minor_value: f32,
) {
    match world.get_element(&matrix) {
        ElementType::Matrix3x3(matrix) => assert_eq!(matrix.minor(row_idx, col_idx), minor_value),
        ElementType::Matrix2x2(_) | ElementType::Matrix4x4(_) => {
            panic!("Only supported on 3x3 matrices")
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "cofactor\\({word}, {int}, {int}) = {int}")]
fn cofactor_equals(
    world: &mut RayTracerWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    cofactor_value: f32,
) {
    match world.get_element(&matrix) {
        ElementType::Matrix2x2(_) => panic!("Not supported"),
        ElementType::Matrix3x3(matrix) => {
            assert_eq!(matrix.cofactor(row_idx, col_idx), cofactor_value)
        }
        ElementType::Matrix4x4(matrix) => {
            assert_eq!(matrix.cofactor(row_idx, col_idx), cofactor_value)
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} is invertible")]
fn matrix_is_invertible(world: &mut RayTracerWorld, matrix: String) {
    assert!(is_invertible(world, matrix));
}

#[then(expr = "{word} is not invertible")]
fn matrix_is_not_invertible(world: &mut RayTracerWorld, matrix: String) {
    assert!(!is_invertible(world, matrix));
}

fn is_invertible(world: &mut RayTracerWorld, matrix: String) -> bool {
    match world.get_element(&matrix) {
        ElementType::Matrix2x2(matrix) => matrix.is_invertible(),
        ElementType::Matrix3x3(matrix) => matrix.is_invertible(),
        ElementType::Matrix4x4(matrix) => matrix.is_invertible(),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word}[{int},{int}] = {float}\\/{float}")]
fn value_in_matrix_equals_fraction(
    world: &mut RayTracerWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    numerator: f32,
    denominator: f32,
) {
    let result = numerator / denominator;

    match world.get_element(&matrix) {
        ElementType::Matrix2x2(matrix) => {
            assert_eq!(matrix.data[row_idx][col_idx], result)
        }
        ElementType::Matrix3x3(matrix) => {
            assert_eq!(matrix.data[row_idx][col_idx], result)
        }
        ElementType::Matrix4x4(matrix) => {
            assert_eq!(matrix.data[row_idx][col_idx], result)
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "inverse\\({word}) is the following {int}x{int} matrix:")]
fn inverse_of_matrix_is_the_following_matrix(
    world: &mut RayTracerWorld,
    matrix: String,
    _row_idx: usize,
    _col_idx: usize,
    step: &Step,
) {
    match world.get_element(&matrix) {
        ElementType::Matrix2x2(_) => panic!("not implemented"),
        ElementType::Matrix3x3(matrix) => assert_eq!(matrix.invert(), Ok(get_matrix::<3, 3>(step))),
        ElementType::Matrix4x4(matrix) => assert_eq!(matrix.invert(), Ok(get_matrix::<4, 4>(step))),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} * inverse\\({word}) = {word}")]
fn matrix_multiplied_by_inverse_of_matrix_equals_matrix(
    world: &mut RayTracerWorld,
    matrix1: String,
    matrix2: String,
    result_matrix: String,
) {
    match (world.get_element(&matrix1), world.get_element(&matrix2)) {
        (ElementType::Matrix2x2(_), ElementType::Matrix2x2(_)) => panic!("not implemented"),
        (ElementType::Matrix3x3(matrix1), ElementType::Matrix3x3(matrix2)) => assert_eq!(
            matrix1 * &matrix2.invert().unwrap(),
            *world.get_matrix3x3(&result_matrix)
        ),
        (ElementType::Matrix4x4(matrix1), ElementType::Matrix4x4(matrix2)) => assert_eq!(
            matrix1 * &matrix2.invert().unwrap(),
            *world.get_matrix4x4(&result_matrix)
        ),
        _ => panic!("no matrix with given size"),
    }
}

fn get_matrix<const ROW_COUNT: usize, const COL_COUNT: usize>(
    step: &Step,
) -> Matrix<ROW_COUNT, COL_COUNT> {
    let mut data = Vec::<[f32; COL_COUNT]>::with_capacity(ROW_COUNT);
    let table = step.table.as_ref().unwrap();
    for row_value in table.rows.iter().map(|row_values| {
        row_values
            .iter()
            .map(|v| v.parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
    }) {
        data.push(row_value.try_into().unwrap());
    }

    Matrix {
        data: data.try_into().unwrap(),
    }
}

fn get_matrix_size(step: &Step) -> usize {
    let table = step.table.as_ref().unwrap();
    table.rows.iter().count()
}

#[then(expr = "{word} = scaling\\({float}, {float}, {float})")]
fn transformation_equals_scaling(
    world: &mut RayTracerWorld,
    transformation: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let actual = world.get_matrix4x4(&transformation);
    let expected = Matrix::new_scaling(x, y, z);
    assert_eq!(actual, &expected);
}

#[then(expr = "{word} = translation\\({float}, {float}, {float})")]
fn transformation_equals_translation(
    world: &mut RayTracerWorld,
    transformation: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let actual = world.get_matrix4x4(&transformation);
    let expected = Matrix::new_translation(x, y, z);
    assert_eq!(actual, &expected);
}
