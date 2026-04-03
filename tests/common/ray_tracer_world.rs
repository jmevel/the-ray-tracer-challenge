use std::collections::HashMap;

use cucumber::World;
use the_ray_tracer_challenge::{Canvas, Matrix, Tuple};

#[derive(Debug, Default, World)]
pub struct RayTracerWorld {
    pub canvases: HashMap<String, Canvas>,
    pub tuples: HashMap<String, Tuple>,
    pub ppms: HashMap<String, String>,

    pub matrices2x2: HashMap<String, Matrix<2, 2>>,
    pub matrices3x3: HashMap<String, Matrix<3, 3>>,
    pub matrices4x4: HashMap<String, Matrix<4, 4>>,
    pub matrices: HashMap<String, (usize, usize)>,
}

// Rust Analyzer thinks all the functions are never used because each file under `tests` is treated as a separate crate
#[allow(dead_code)]
impl RayTracerWorld {
    pub fn get_tuple(&self, tuple: &str) -> &Tuple {
        self.tuples
            .get(tuple)
            .expect(format!("{tuple} does not exist").as_str())
    }

    pub fn get_canvas(&self, canvas: &str) -> &Canvas {
        self.canvases
            .get(canvas)
            .expect(format!("{canvas} does not exist").as_str())
    }

    pub fn get_mut_canvas(&mut self, canvas: &str) -> &mut Canvas {
        self.canvases
            .get_mut(canvas)
            .expect(format!("{canvas} does not exist").as_str())
    }

    pub fn get_ppm(&self, ppm: &str) -> &str {
        self.ppms
            .get(ppm)
            .expect(format!("{ppm} does not exist").as_str())
    }

    pub fn get_matrix2x2(&self, matrix: &str) -> &Matrix<2, 2> {
        self.matrices2x2
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }

    pub fn get_matrix3x3(&self, matrix: &str) -> &Matrix<3, 3> {
        self.matrices3x3
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }

    pub fn get_matrix4x4(&self, matrix: &str) -> &Matrix<4, 4> {
        self.matrices4x4
            .get(matrix)
            .expect(format!("{matrix} does not exist").as_str())
    }

    pub fn add_matrix2x2(&mut self, matrix_name: String, matrix: Matrix<2, 2>) {
        self.matrices2x2.insert(matrix_name.clone(), matrix);
        self.matrices.insert(matrix_name.clone(), (2, 2));
    }

    pub fn add_matrix3x3(&mut self, matrix_name: String, matrix: Matrix<3, 3>) {
        self.matrices3x3.insert(matrix_name.clone(), matrix);
        self.matrices.insert(matrix_name.clone(), (3, 3));
    }

    pub fn add_matrix4x4(&mut self, matrix_name: String, matrix: Matrix<4, 4>) {
        self.matrices4x4.insert(matrix_name.clone(), matrix);
        self.matrices.insert(matrix_name.clone(), (4, 4));
    }
}
