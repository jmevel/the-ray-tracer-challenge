use crate::Tuple;
use crate::float::float_equals;
use core::fmt;
use duplicate::duplicate_item;
use std::fmt::Debug;
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Matrix<const ROW_COUNT: usize, const COL_COUNT: usize> {
    pub data: [[f32; COL_COUNT]; ROW_COUNT],
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Debug for Matrix<ROW_COUNT, COL_COUNT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> fmt::Display for Matrix<ROW_COUNT, COL_COUNT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = writeln!(f, "Matrix:");
        for row in 0..ROW_COUNT {
            for col in 0..COL_COUNT {
                _ = write!(f, "| {:<18}", &self.data[col][row]);
            }
            _ = writeln!(f, "|");
        }
        Ok(())
    }
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Matrix<ROW_COUNT, COL_COUNT> {
    pub fn identity_matrix() -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut data: [[f32; COL_COUNT]; ROW_COUNT] = [[0f32; COL_COUNT]; ROW_COUNT];
        for i in 0..ROW_COUNT {
            data[i][i] = 1f32;
        }

        Matrix { data }
    }

    pub fn new_translation(x: f32, y: f32, z: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut result = Matrix::identity_matrix();
        result.data[0][3] = result.data[0][3] + x;
        result.data[1][3] = result.data[1][3] + y;
        result.data[2][3] = result.data[2][3] + z;
        result
    }

    pub fn new_scaling(x: f32, y: f32, z: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut result = Matrix::identity_matrix();
        result.data[0][0] = result.data[0][0] * x;
        result.data[1][1] = result.data[1][1] * y;
        result.data[2][2] = result.data[2][2] * z;
        result
    }

    pub fn new_rotation_x(radians: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut result = Matrix::identity_matrix();
        result.data[1][1] = radians.cos();
        result.data[1][2] = -radians.sin();
        result.data[2][1] = radians.sin();
        result.data[2][2] = radians.cos();
        result
    }

    pub fn new_rotation_y(radians: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut result = Matrix::identity_matrix();
        result.data[0][0] = radians.cos();
        result.data[0][2] = radians.sin();
        result.data[2][0] = -radians.sin();
        result.data[2][2] = radians.cos();
        result
    }

    pub fn new_rotation_z(radians: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut result = Matrix::identity_matrix();
        result.data[0][0] = radians.cos();
        result.data[0][1] = -radians.sin();
        result.data[1][0] = radians.sin();
        result.data[1][1] = radians.cos();
        result
    }

    pub fn new_shearing(
        xy: f32,
        xz: f32,
        yx: f32,
        yz: f32,
        zx: f32,
        zy: f32,
    ) -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut result = Matrix::identity_matrix();
        result.data[0][1] = xy;
        result.data[0][2] = xz;
        result.data[1][0] = yx;
        result.data[1][2] = yz;
        result.data[2][0] = zx;
        result.data[2][1] = zy;
        result
    }

    pub fn transpose(&self) -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut data = self.data.clone();
        for idx in 0..ROW_COUNT {
            data[idx] = self.get_column(idx);
        }

        Matrix { data }
    }

    pub fn submatrix(
        &self,
        row_idx: usize,
        col_idx: usize,
    ) -> Matrix<{ ROW_COUNT - 1 }, { COL_COUNT - 1 }> {
        let data: [[f32; COL_COUNT - 1]; ROW_COUNT - 1] = self
            .data
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx != &row_idx)
            .map(|(_, row)| {
                let filtered_row: Vec<f32> = row
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| idx != &col_idx)
                    .map(|(_, &value)| value)
                    .collect();
                filtered_row.try_into().unwrap()
            })
            .collect::<Vec<[f32; COL_COUNT - 1]>>()
            .try_into()
            .unwrap();

        Matrix { data }
    }

    fn get_column(&self, col_idx: usize) -> [f32; COL_COUNT] {
        self.data
            .iter()
            .map(|row: &[f32; COL_COUNT]| row[col_idx])
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap()
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let translation = Matrix::new_translation(x, y, z);
        &translation * self
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let scaling = Matrix::new_scaling(x, y, z);
        &scaling * self
    }

    pub fn rotate_x(&self, radians: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let rotation_x = Matrix::new_rotation_x(radians);
        &rotation_x * self
    }

    pub fn rotate_y(&self, radians: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let rotation_y = Matrix::new_rotation_y(radians);
        &rotation_y * self
    }

    pub fn rotate_z(&self, radians: f32) -> Matrix<ROW_COUNT, COL_COUNT> {
        let rotation_z = Matrix::new_rotation_z(radians);
        &rotation_z * self
    }

    pub fn shear(
        &self,
        xy: f32,
        xz: f32,
        yx: f32,
        yz: f32,
        zx: f32,
        zy: f32,
    ) -> Matrix<ROW_COUNT, COL_COUNT> {
        let shear = Matrix::new_shearing(xy, xz, yx, yz, zx, zy);
        &shear * self
    }
}

impl Matrix<2, 2> {
    pub fn determinant(&self) -> f32 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0f32
    }
}

#[duplicate_item(
  row_count  col_count;
  [ 3 ]    [ 3 ];
  [ 4 ]    [ 4 ];
)]
impl Matrix<row_count, col_count> {
    pub fn determinant(&self) -> f32 {
        let row_idx = 0; // we take any row or column, it doesn't really matter
        self.data[row_idx]
            .iter()
            .enumerate()
            .fold(0f32, |acc, (col_idx, &value)| {
                acc + value * self.cofactor(row_idx, col_idx)
            })
    }

    pub fn minor(&self, row_idx: usize, col_idx: usize) -> f32 {
        let submatrix = self.submatrix(row_idx, col_idx);
        submatrix.determinant()
    }

    pub fn cofactor(&self, row_idx: usize, col_idx: usize) -> f32 {
        let minor = self.minor(row_idx, col_idx);
        (-1i32).pow((row_idx + col_idx) as u32) as f32 * minor
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0f32
    }

    pub fn invert(&self) -> Result<Matrix<row_count, col_count>, String> {
        if !self.is_invertible() {
            return Err("Matrix is not revertible".to_string());
        }

        let data: [[f32; row_count]; col_count] = self
            .data
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_idx, _)| self.cofactor(row_idx, col_idx) / self.determinant())
                    .collect::<Vec<f32>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f32; row_count]>>()
            .try_into()
            .unwrap();

        Ok(Matrix { data }.transpose())
    }
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Eq for Matrix<ROW_COUNT, COL_COUNT> {}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> PartialEq for Matrix<ROW_COUNT, COL_COUNT> {
    fn eq(&self, other: &Self) -> bool {
        !self.data.iter().enumerate().any(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .any(|(col_idx, value)| !float_equals(value, &other.data[row_idx][col_idx]))
        })
    }
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Mul for Matrix<ROW_COUNT, COL_COUNT> {
    type Output = Matrix<ROW_COUNT, COL_COUNT>;

    fn mul(self, rhs: Matrix<ROW_COUNT, COL_COUNT>) -> Self::Output {
        &self * &rhs
    }
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Mul<&Matrix<ROW_COUNT, COL_COUNT>>
    for &Matrix<ROW_COUNT, COL_COUNT>
{
    type Output = Matrix<ROW_COUNT, COL_COUNT>;

    fn mul(self, rhs: &Matrix<ROW_COUNT, COL_COUNT>) -> Self::Output {
        let mut data: [[f32; COL_COUNT]; ROW_COUNT] = self.data;
        for row_idx in 0..ROW_COUNT {
            for col_idx in 0..COL_COUNT {
                data[row_idx][col_idx] = (0..ROW_COUNT).fold(0f32, |acc, idx| {
                    acc + self.data[row_idx][idx] * rhs.data[idx][col_idx]
                });
            }
        }

        Matrix { data }
    }
}

impl<T: Tuple> Mul<&T> for &Matrix<4, 4> {
    type Output = T;

    fn mul(self, other: &T) -> Self::Output {
        let x = self.data[0][0] * other.x()
            + self.data[0][1] * other.y()
            + self.data[0][2] * other.z()
            + self.data[0][3] * other.w();
        let y = self.data[1][0] * other.x()
            + self.data[1][1] * other.y()
            + self.data[1][2] * other.z()
            + self.data[1][3] * other.w();
        let z = self.data[2][0] * other.x()
            + self.data[2][1] * other.y()
            + self.data[2][2] * other.z()
            + self.data[2][3] * other.w();
        let w = self.data[3][0] * other.x()
            + self.data[3][1] * other.y()
            + self.data[3][2] * other.z()
            + self.data[3][3] * other.w();

        Tuple::new(x, y, z, w)
    }
}

impl<T: Tuple> Mul<T> for Matrix<4, 4> {
    type Output = T;

    fn mul(self, other: T) -> Self::Output {
        &self * &other
    }
}
