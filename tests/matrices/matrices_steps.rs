use cucumber::gherkin::Step;
use cucumber::{given, then, World};
use std::collections::HashMap;
use the_ray_tracer_challenge::matrix::Matrix;

#[derive(Debug, Default, World)]
pub struct MatrixWorld {
    matrices4x4: HashMap<String, Matrix<4, 4>>,
}

#[given(expr = "the following 4x4 matrix {word}:")]
fn the_following_4x4_matrix(world: &mut MatrixWorld, matrix: String, step: &Step) {
    world.matrices4x4.insert(matrix, get_matrix4x4(step));
}

#[then(expr = "{word}[{int},{int}] = {float}")]
fn matrix_equals(
    world: &mut MatrixWorld,
    matrix: String,
    row_idx: usize,
    col_idx: usize,
    value: f32,
) {
    assert_eq!(world.get_matrix4x4(matrix).data[row_idx][col_idx], value);
}

fn get_matrix4x4(step: &Step) -> Matrix<4, 4> {
    let mut data: [[f32; 4]; 4] = Default::default();
    let table = step.table.as_ref().unwrap();
    for (row_idx, row_value) in table.rows.iter().enumerate().map(|(idx, row_values)| {
        (
            idx,
            row_values
                .iter()
                .map(|v| v.parse::<f32>().unwrap())
                .collect::<Vec<f32>>(),
        )
    }) {
        data[row_idx] = row_value.try_into().unwrap();
    }

    Matrix { data }
}

impl MatrixWorld {
    fn get_matrix4x4(&self, matrix: String) -> &Matrix<4, 4> {
        self.matrices4x4
            .get(&matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }
}
