use cucumber::gherkin::Step;
use cucumber::{World, given, then};
use std::collections::HashMap;
use the_ray_tracer_challenge::Tuple;
use the_ray_tracer_challenge::matrix::Matrix;

#[derive(Debug, Default, World)]
pub struct MatrixWorld {
    matrices2x2: HashMap<String, Matrix<2, 2>>,
    matrices3x3: HashMap<String, Matrix<3, 3>>,
    matrices4x4: HashMap<String, Matrix<4, 4>>,
    matrices: HashMap<String, (usize, usize)>,
    tuples: HashMap<String, Tuple>,
}

#[given(expr = "the following 2x2 matrix {word}:")]
fn the_following_2x2_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    world
        .matrices2x2
        .insert(matrix.clone(), get_matrix::<2, 2>(step));
    world.matrices.insert(matrix, (2, 2));
}

#[given(expr = "the following 3x3 matrix {word}:")]
fn the_following_3x3_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    world
        .matrices3x3
        .insert(matrix.clone(), get_matrix::<3, 3>(step));
    world.matrices.insert(matrix, (3, 3));
}

#[given(expr = "the following 4x4 matrix {word}:")]
fn the_following_4x4_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    world
        .matrices4x4
        .insert(matrix.clone(), get_matrix::<4, 4>(step));
    world.matrices.insert(matrix, (4, 4));
}

#[given(expr = "the following matrix {word}:")]
fn the_following_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    match get_matrix_size(step) {
        2 => {
            world
                .matrices2x2
                .insert(matrix.clone(), get_matrix::<2, 2>(step));
            world.matrices.insert(matrix, (2, 2));
        }
        3 => {
            world
                .matrices3x3
                .insert(matrix.clone(), get_matrix::<3, 3>(step));
            world.matrices.insert(matrix, (3, 3));
        }
        4 => {
            world
                .matrices4x4
                .insert(matrix.clone(), get_matrix::<4, 4>(step));
            world.matrices.insert(matrix, (4, 4));
        }
        _ => panic!("matrix size not supported"),
    }
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn tuple_is(world: &mut MatrixWorld, tuple: String, x: f32, y: f32, z: f32, w: f32) {
    world.tuples.insert(tuple, Tuple::new(x, y, z, w));
}

#[given(expr = "{word} ← transpose\\(identity_matrix)")]
fn transpose_identity_matrix(world: &mut MatrixWorld, matrix: String) {
    let transposed_identity_matrix: Matrix<4, 4> = Matrix::identity_matrix().transpose();
    world.matrices4x4.insert(matrix, transposed_identity_matrix);
}

#[given(expr = "{word} ← submatrix\\({word}, {int}, {int})")]
fn matrix_is_submatrix(
    world: &mut MatrixWorld,
    new_matrix_name: String,
    initial_matrix_name: String,
    row_idx: usize,
    col_idx: usize,
) {
    let initial_matrix = match world
        .matrices
        .get(&initial_matrix_name)
        .expect(format!("{initial_matrix_name} does not exist").as_str())
    {
        (3, 3) => world.get_matrix3x3(&initial_matrix_name),
        (2, 2) | (4, 4) => panic!("Only supported on 3x3 matrices"),
        _ => panic!("no matrix with given size"),
    };

    let new_matrix: Matrix<2, 2> = initial_matrix.submatrix(row_idx, col_idx);
    world
        .matrices2x2
        .insert(new_matrix_name.clone(), new_matrix);
    world.matrices.insert(new_matrix_name, (2, 2));
}

#[given(expr = "{word} ← inverse\\({word})")]
fn matrix_is_inverse_of_matrix(
    world: &mut MatrixWorld,
    new_matrix_name: String,
    initial_matrix_name: String,
) {
    match world
        .matrices
        .get(&initial_matrix_name)
        .expect(format!("{initial_matrix_name} does not exist").as_str())
    {
        (2, 2) => panic!("can't invert a 2x2 matrix"),
        (3, 3) => {
            let initial_matrix = world.get_matrix3x3(&initial_matrix_name);
            world
                .matrices3x3
                .insert(new_matrix_name.clone(), initial_matrix.invert().unwrap());
            world.matrices.insert(new_matrix_name, (3, 3));
        }
        (4, 4) => {
            let initial_matrix = world.get_matrix4x4(&initial_matrix_name);
            world
                .matrices4x4
                .insert(new_matrix_name.clone(), initial_matrix.invert().unwrap());
            world.matrices.insert(new_matrix_name, (4, 4));
        }
        _ => panic!("no matrix with given size"),
    }
}

#[given(expr = "{word} ← {word} * {word}")]
fn matrix_is_matrix_multiplied_by_matrix(
    world: &mut MatrixWorld,
    product_matrix: String,
    matrix1: String,
    matrix2: String,
) {
    match world
        .matrices
        .get(&matrix1)
        .expect(format!("{matrix1} does not exist").as_str())
    {
        (2, 2) => {
            world.matrices2x2.insert(
                product_matrix.clone(),
                world.get_matrix2x2(&matrix1) * world.get_matrix2x2(&matrix2),
            );
            world.matrices.insert(product_matrix, (2, 2));
        }
        (3, 3) => {
            world.matrices3x3.insert(
                product_matrix.clone(),
                world.get_matrix3x3(&matrix1) * world.get_matrix3x3(&matrix2),
            );
            world.matrices.insert(product_matrix, (3, 3));
        }
        (4, 4) => {
            world.matrices4x4.insert(
                product_matrix.clone(),
                world.get_matrix4x4(&matrix1) * world.get_matrix4x4(&matrix2),
            );
            world.matrices.insert(product_matrix, (4, 4));
        }
        _ => panic!("no matrix with given size"),
    };
}

#[then(expr = "{word}[{int},{int}] = {float}")]
fn matrix_index_equals(
    world: &mut MatrixWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    value: f32,
) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => assert_eq!(world.get_matrix2x2(&matrix).data[row_idx][col_idx], value),
        (3, 3) => assert_eq!(world.get_matrix3x3(&matrix).data[row_idx][col_idx], value),
        (4, 4) => assert_eq!(world.get_matrix4x4(&matrix).data[row_idx][col_idx], value),
        _ => panic!("no matrix with given size"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) = ([a-zA-Z0-9]*)$")]
fn matrix_equals_matrix(world: &mut MatrixWorld, matrix1: String, matrix2: String) {
    match world
        .matrices
        .get(&matrix1)
        .expect(format!("{matrix1} does not exist").as_str())
    {
        (2, 2) => assert_eq!(world.get_matrix2x2(&matrix1), world.get_matrix2x2(&matrix2)),
        (3, 3) => assert_eq!(world.get_matrix3x3(&matrix1), world.get_matrix3x3(&matrix2)),
        (4, 4) => assert_eq!(world.get_matrix4x4(&matrix1), world.get_matrix4x4(&matrix2)),
        _ => panic!("no matrix with given size"),
    }
}

#[then(regex = r"^([a-zA-Z0-9]*) != ([a-zA-Z0-9]*)$")]
fn matrix_does_not_equal_matrix(world: &mut MatrixWorld, matrix1: String, matrix2: String) {
    match world
        .matrices
        .get(&matrix1)
        .expect(format!("{matrix1} does not exist").as_str())
    {
        (2, 2) => assert_ne!(world.get_matrix2x2(&matrix1), world.get_matrix2x2(&matrix2)),
        (3, 3) => assert_ne!(world.get_matrix3x3(&matrix1), world.get_matrix3x3(&matrix2)),
        (4, 4) => assert_ne!(world.get_matrix4x4(&matrix1), world.get_matrix4x4(&matrix2)),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} * {word} is the following {int}x{int} matrix:")]
fn matrix_multiplied_by_matrix_is_the_following_matrix(
    world: &mut MatrixWorld,
    matrix1: String,
    matrix2: String,
    row_size: usize,
    col_size: usize,
    step: &Step,
) {
    match (row_size, col_size) {
        (2, 2) | (3, 3) => panic!("step definition not implemented (not needed)"),
        (4, 4) => {
            let matrix1 = world.matrices4x4.get(&matrix1).unwrap();
            let matrix2 = world.matrices4x4.get(&matrix2).unwrap();

            let expected = get_matrix::<4, 4>(step);
            let actual = matrix1 * matrix2;

            assert_eq!(actual, expected);
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(
    regex = r"^([a-zA-Z0-9]*) is the following ((?:-?\d+)|(?:\d+))x((?:-?\d+)|(?:\d+)) matrix:$"
)]
fn matrix_is_the_following_matrix(
    world: &mut MatrixWorld,
    matrix: String,
    row_size: usize,
    col_size: usize,
    step: &Step,
) {
    match (row_size, col_size) {
        (2, 2) => panic!("step definition not implemented (not needed)"),
        (3, 3) => {
            let matrix = world.matrices3x3.get(&matrix).unwrap();
            let expected = get_matrix::<3, 3>(step);
            assert_eq!(matrix, &expected);
        }
        (4, 4) => {
            let matrix = world.matrices4x4.get(&matrix).unwrap();
            let expected = get_matrix::<4, 4>(step);
            assert_eq!(matrix, &expected);
        }
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} * {word} = tuple\\({float}, {float}, {float}, {float})")]
fn matrix_multiplied_by_tuple_equals_tuple(
    world: &mut MatrixWorld,
    matrix: String,
    tuple: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let tuple = world.get_tuple(&tuple);
    let expected = Tuple::new(x, y, z, w);

    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) | (3, 3) => panic!("not supported"),
        (4, 4) => assert_eq!(world.get_matrix4x4(&matrix) * tuple, expected),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} * identity_matrix = {word}")]
fn matrix_multiplied_by_identity_matrix_equals_matrix(
    world: &mut MatrixWorld,
    matrix: String,
    _matrix: String,
) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => assert_eq!(
            world.get_matrix2x2(&matrix) * &Matrix::identity_matrix(),
            *world.get_matrix2x2(&matrix)
        ),
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix) * &Matrix::identity_matrix(),
            *world.get_matrix3x3(&matrix)
        ),
        (4, 4) => assert_eq!(
            world.get_matrix4x4(&matrix) * &Matrix::identity_matrix(),
            *world.get_matrix4x4(&matrix)
        ),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "identity_matrix * {word} = {word}")]
fn identity_matrix_multiplied_by_tuple_equals_tuple(
    world: &mut MatrixWorld,
    tuple: String,
    _tuple: String,
) {
    let tuple = world.get_tuple(&tuple);
    let identity_matrix: Matrix<4, 4> = Matrix::identity_matrix();
    assert_eq!(&identity_matrix * tuple, *tuple);
}

#[then(expr = "transpose\\({word}) is the following matrix:")]
fn transpose_matrix_is_the_following_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => assert_eq!(
            world.get_matrix2x2(&matrix).transpose(),
            get_matrix::<2, 2>(step)
        ),
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix).transpose(),
            get_matrix::<3, 3>(step)
        ),
        (4, 4) => assert_eq!(
            world.get_matrix4x4(&matrix).transpose(),
            get_matrix::<4, 4>(step)
        ),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} = identity_matrix")]
fn matrix_equals_identity_matrix(world: &mut MatrixWorld, matrix: String) {
    let matrix = world.get_matrix4x4(&matrix);
    let identity_matrix: Matrix<4, 4> = Matrix::identity_matrix();
    assert_eq!(&identity_matrix, matrix);
}

#[then(expr = "determinant\\({word}) = {int}")]
fn determinant_equals(world: &mut MatrixWorld, matrix: String, determinant: f32) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => assert_eq!(
            world.get_matrix2x2(&matrix).determinant(),
            f32::from(determinant)
        ),
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix).determinant(),
            f32::from(determinant)
        ),
        (4, 4) => assert_eq!(
            world.get_matrix4x4(&matrix).determinant(),
            f32::from(determinant)
        ),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "submatrix\\({word}, {int}, {int}) is the following {int}x{int} matrix:")]
fn submatrix_is_the_following_matrix(
    world: &mut MatrixWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    _row_size: usize,
    _col_size: usize,
    step: &Step,
) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => panic!("can't make a submatrix of a 2x2 matrix"),
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix).submatrix(row_idx, col_idx),
            get_matrix::<2, 2>(step)
        ),
        (4, 4) => assert_eq!(
            world.get_matrix4x4(&matrix).submatrix(row_idx, col_idx),
            get_matrix::<3, 3>(step)
        ),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "minor\\({word}, {int}, {int}) = {int}")]
fn minor_equals(
    world: &mut MatrixWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    minor_value: f32,
) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix).minor(row_idx, col_idx),
            minor_value
        ),
        (2, 2) | (4, 4) => panic!("Only supported on 3x3 matrices"),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "cofactor\\({word}, {int}, {int}) = {int}")]
fn cofactor_equals(
    world: &mut MatrixWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    cofactor_value: f32,
) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix).cofactor(row_idx, col_idx),
            cofactor_value
        ),
        (4, 4) => assert_eq!(
            world.get_matrix4x4(&matrix).cofactor(row_idx, col_idx),
            cofactor_value
        ),
        (2, 2) => panic!("Only supported on 3x3 matrices"),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} is invertible")]
fn matrix_is_invertible(world: &mut MatrixWorld, matrix: String) {
    assert!(is_invertible(world, matrix));
}

#[then(expr = "{word} is not invertible")]
fn matrix_is_not_invertible(world: &mut MatrixWorld, matrix: String) {
    assert!(!is_invertible(world, matrix));
}

fn is_invertible(world: &mut MatrixWorld, matrix: String) -> bool {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => world.get_matrix2x2(&matrix).is_invertible(),
        (3, 3) => world.get_matrix3x3(&matrix).is_invertible(),
        (4, 4) => world.get_matrix4x4(&matrix).is_invertible(),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word}[{int},{int}] = {float}\\/{float}")]
fn value_in_matrix_equals_fraction(
    world: &mut MatrixWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    numerator: f32,
    denominator: f32,
) {
    let result = numerator / denominator;

    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => assert_eq!(world.get_matrix2x2(&matrix).data[row_idx][col_idx], result),
        (3, 3) => assert_eq!(world.get_matrix3x3(&matrix).data[row_idx][col_idx], result),
        (4, 4) => assert_eq!(world.get_matrix4x4(&matrix).data[row_idx][col_idx], result),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "inverse\\({word}) is the following {int}x{int} matrix:")]
fn inverse_of_matrix_is_the_following_matrix(
    world: &mut MatrixWorld,
    matrix: String,
    _row_idx: usize,
    _col_idx: usize,
    step: &Step,
) {
    match world
        .matrices
        .get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2, 2) => panic!("not implemented"),
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix).invert(),
            Ok(get_matrix::<3, 3>(step))
        ),
        (4, 4) => assert_eq!(
            world.get_matrix4x4(&matrix).invert(),
            Ok(get_matrix::<4, 4>(step))
        ),
        _ => panic!("no matrix with given size"),
    }
}

#[then(expr = "{word} * inverse\\({word}) = {word}")]
fn matrix_multiplied_by_inverse_of_matrix_equals_matrix(
    world: &mut MatrixWorld,
    matrix1: String,
    matrix2: String,
    result_matrix: String,
) {
    match world
        .matrices
        .get(&matrix1)
        .expect(format!("{matrix1} does not exist").as_str())
    {
        (2, 2) => panic!("not implemented"),
        (3, 3) => assert_eq!(
            world.get_matrix3x3(&matrix1) * &world.get_matrix3x3(&matrix2).invert().unwrap(),
            *world.get_matrix3x3(&result_matrix)
        ),
        (4, 4) => assert_eq!(
            world.get_matrix4x4(&matrix1) * &world.get_matrix4x4(&matrix2).invert().unwrap(),
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

impl MatrixWorld {
    fn get_matrix2x2(&self, matrix: &str) -> &Matrix<2, 2> {
        self.matrices2x2
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }
    fn get_matrix3x3(&self, matrix: &str) -> &Matrix<3, 3> {
        self.matrices3x3
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }
    fn get_matrix4x4(&self, matrix: &str) -> &Matrix<4, 4> {
        self.matrices4x4
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }
    fn get_tuple(&self, tuple: &str) -> &Tuple {
        self.tuples
            .get(tuple)
            .expect(format!("{tuple} does not exist").as_str())
    }
}
