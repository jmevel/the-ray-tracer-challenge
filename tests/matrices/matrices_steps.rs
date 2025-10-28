use cucumber::gherkin::Step;
use cucumber::{given, then, World};
use std::collections::HashMap;
use the_ray_tracer_challenge::matrix::Matrix;

#[derive(Debug, Default, World)]
pub struct MatrixWorld {
    matrices2x2: HashMap<String, Matrix<2, 2>>,
    matrices3x3: HashMap<String, Matrix<3, 3>>,
    matrices4x4: HashMap<String, Matrix<4, 4>>,
    matrices: HashMap<String, (usize, usize)>
}

#[given(expr = "the following 2x2 matrix {word}:")]
fn the_following_2x2_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    world.matrices2x2.insert(matrix.clone(), get_matrix::<2, 2>(step));
    world.matrices.insert(matrix, (2,2));
}

#[given(expr = "the following 3x3 matrix {word}:")]
fn the_following_3x3_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    world.matrices3x3.insert(matrix.clone(), get_matrix::<3, 3>(step));
    world.matrices.insert(matrix, (3,3));
}

#[given(expr = "the following 4x4 matrix {word}:")]
fn the_following_4x4_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    world.matrices4x4.insert(matrix.clone(), get_matrix::<4, 4>(step));
    world.matrices.insert(matrix, (4,4));
}

#[then(expr = "{word}[{int},{int}] = {float}")]
fn matrix_equals(
    world: &mut MatrixWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    value: f32,
) {
    match world.matrices.get(&matrix)
        .expect(format!("{matrix} does not exist").as_str())
    {
        (2,2) => assert_eq!(world.get_matrix2x2(matrix).data[row_idx][col_idx], value),
        (3,3) => assert_eq!(world.get_matrix3x3(matrix).data[row_idx][col_idx], value),
        (4,4) => assert_eq!(world.get_matrix4x4(matrix).data[row_idx][col_idx], value),
        _ => panic!("no matrix with given size")
    }
}

fn get_matrix<const ROW_COUNT: usize, const COL_COUNT: usize>(step: &Step) -> Matrix<ROW_COUNT, COL_COUNT> {
    let mut data = Vec::<[f32; ROW_COUNT]>::with_capacity(COL_COUNT);
    let table = step.table.as_ref().unwrap();
    for (_, row_value) in table.rows.iter().enumerate().map(|(idx, row_values)| {
        (
            idx,
            row_values
                .iter()
                .map(|v| v.parse::<f32>().unwrap())
                .collect::<Vec<f32>>(),
        )
    }) {
        data.push(row_value.try_into().unwrap());
    }

    Matrix { data: data.try_into().unwrap() }
}

impl MatrixWorld {
    fn get_matrix2x2(&self, matrix: String) -> &Matrix<2, 2> {
        self.matrices2x2
            .get(&matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }
    fn get_matrix3x3(&self, matrix: String) -> &Matrix<3, 3> {
        self.matrices3x3
            .get(&matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }
    fn get_matrix4x4(&self, matrix: String) -> &Matrix<4, 4> {
        self.matrices4x4
            .get(&matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }
}
