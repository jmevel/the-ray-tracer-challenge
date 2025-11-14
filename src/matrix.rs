use crate::Tuple;
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix<const ROW_COUNT: usize, const COL_COUNT: usize> {
    pub data: [[f32; COL_COUNT]; ROW_COUNT],
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Matrix<ROW_COUNT, COL_COUNT> {
    pub fn identity_matrix() -> Matrix<ROW_COUNT, COL_COUNT> {
        let mut data: [[f32; COL_COUNT]; ROW_COUNT] = [[0f32; COL_COUNT]; ROW_COUNT];
        for i in 0..ROW_COUNT {
            data[i][i] = 1f32;
        }

        Matrix { data }
    }
}

impl Eq for Matrix<4, 4> {}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> PartialEq for Matrix<ROW_COUNT, COL_COUNT> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<const ROW_COUNT: usize, const COL_COUNT: usize> Mul for &Matrix<ROW_COUNT, COL_COUNT> {
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

impl Mul<&Tuple> for &Matrix<4, 4> {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Self::Output {
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

impl Mul<&Tuple> for Matrix<4,4> {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Self::Output {
        &self * other
    }
}
